use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;
use yew::prelude::*;

type SynchiStore = HashMap<&'static str, Rc<RefCell<HashMap<usize, DataStatus>>>>;
type SynchiSubscriber = Callback<Box<dyn Any>>;

enum DataStatus {
    Free(Box<dyn Any>),
    Claimed(SynchiSubscriber, Box<dyn Any>),
}

thread_local! {
    static SYNCHI: Rc<RefCell<SynchiStore>> = Rc::new(RefCell::new(HashMap::new()));
    static SYNCHI_COUNTERS: RefCell<HashMap<&'static str, usize>> = RefCell::new(HashMap::new());
    static SYNCHI_SUBSCRIBERS: RefCell<HashMap<&'static str, Vec<SynchiSubscriber>>> =
        RefCell::new(HashMap::new());
}

pub trait Merge {
    fn merge(&self, other: &Self) -> Self;
}

fn register_channel<T>(name: &'static str, data: T) -> Option<usize>
where
    T: Any,
{
    SYNCHI.with(|store| {
        let mut store = store.borrow_mut();

        if let Some(existing_channel) = store.get(name) {
            let mut channel = existing_channel.borrow_mut();

            let index = SYNCHI_COUNTERS.with(|counters| {
                let mut counters = counters.borrow_mut();

                if let Some(counter) = counters.get_mut(name) {
                    *counter += 1;
                    *counter
                } else {
                    counters.insert(name, 0);
                    0
                }
            });

            channel.insert(index, DataStatus::Free(Box::new(data)));
            return Some(index);
        }

        SYNCHI_COUNTERS.with(|counters| {
            let mut counters = counters.borrow_mut();
            counters.insert(name, 0);
        });

        let new_channel = Rc::new(RefCell::new(HashMap::<usize, DataStatus>::from_iter(vec![
            (0, DataStatus::Free(Box::new(data))),
        ])));

        store.insert(name, new_channel);
        Some(0)
    })
}

fn unregister_channel(name: &'static str, index: usize) -> usize {
    SYNCHI.with(|store| {
        let store = store.borrow_mut();

        if let Some(channel) = store.get(name) {
            let mut channel = channel.borrow_mut();
            channel.remove(&index);

            channel.len()
        } else {
            panic!("Failed to get SYNCHI channel");
        }
    })
}

fn subscribe_to_channel<T>(name: &'static str, indexes: Vec<usize>, callback: SynchiSubscriber)
where
    T: Any + Clone + Debug + Default + Merge,
{
    let mut no_new_subscriber = false;

    SYNCHI_SUBSCRIBERS.with(|subscribers| {
        let mut subscribers = subscribers.borrow_mut();

        if let Some(existing_subscribers) = subscribers.get_mut(name) {
            for existing_subscriber in existing_subscribers.iter() {
                if existing_subscriber == &callback {
                    no_new_subscriber = true;
                    return;
                }
            }

            existing_subscribers.push(callback.clone());
        } else {
            subscribers.insert(name, vec![callback.clone()]);
        }
    });

    if no_new_subscriber {
        return;
    }

    SYNCHI.with(|store| {
        let store = store.borrow_mut();

        if let Some(channel) = store.get(name) {
            let mut channel = channel.borrow_mut();
            let mut merged_data = T::default();

            for index in indexes {
                if let Some(data_status) = channel.get_mut(&index) {
                    match data_status {
                        DataStatus::Free(data) => {
                            if let Some(data) = data.downcast_ref::<T>() {
                                merged_data = merged_data.merge(data);

                                *data_status =
                                    DataStatus::Claimed(callback.clone(), Box::new(data.clone()));
                            }
                        }
                        DataStatus::Claimed(subscriber, data) => {
                            if *subscriber == callback {
                                if let Some(data) = data.downcast_ref::<T>() {
                                    merged_data = merged_data.merge(data);
                                }
                            }
                        }
                    }
                }
            }

            callback.emit(Box::new(merged_data));
        }
    });
}

fn unsubscribe_from_channel<T>(name: &'static str, callback: SynchiSubscriber)
where
    T: Any + Clone + Debug + Default + Merge,
{
    SYNCHI_SUBSCRIBERS.with(|subscribers| {
        let mut subscribers = subscribers.borrow_mut();

        if let Some(existing_subscribers) = subscribers.get_mut(name) {
            existing_subscribers.retain(|subscriber| subscriber != &callback);
        }
    });

    SYNCHI.with(|store| {
        let store = store.borrow_mut();

        if let Some(channel) = store.get(name) {
            let mut channel = channel.borrow_mut();

            for index in 0..channel.len() {
                if let Some(data_status) = channel.get_mut(&index) {
                    if let DataStatus::Claimed(subscriber, data) = data_status {
                        if *subscriber == callback {
                            if let Some(data) = data.downcast_ref::<T>() {
                                *data_status = DataStatus::Free(Box::new(data.clone()));
                            }
                        }
                    }
                }
            }
        }
    });
}

fn notify_subscriber<T>(name: &'static str, callback: SynchiSubscriber)
where
    T: Any + Clone + Debug + Default + Merge,
{
    SYNCHI.with(|store| {
        let store = store.borrow_mut();

        if let Some(channel) = store.get(name) {
            let mut channel = channel.borrow_mut();
            let mut merged_data = T::default();

            for index in 0..channel.len() {
                if let Some(DataStatus::Claimed(subscriber, data)) = channel.get_mut(&index) {
                    if *subscriber == callback {
                        if let Some(data) = data.downcast_ref::<T>() {
                            merged_data = merged_data.merge(data);
                        }
                    }
                }
            }

            callback.emit(Box::new(merged_data));
        }
    });
}

#[derive(Debug, Clone, PartialEq)]
pub struct SynchiChannel<T>
where
    T: Any + Clone + Default,
{
    pub name: &'static str,
    pub index: usize,
    _marker: std::marker::PhantomData<T>,
}

impl<T> Drop for SynchiChannel<T>
where
    T: Any + Clone + Default,
{
    fn drop(&mut self) {
        let channel_len = unregister_channel(self.name, self.index);

        if channel_len == 0 {
            SYNCHI.with(|store| {
                let mut store = store.borrow_mut();

                if let Some(channel) = store.remove(self.name) {
                    drop(channel);
                }
            });

            SYNCHI_COUNTERS.with(|counters| {
                let mut counters = counters.borrow_mut();
                counters.remove(self.name);
            });

            SYNCHI_SUBSCRIBERS.with(|subscribers| {
                let mut subscribers = subscribers.borrow_mut();
                subscribers.remove(self.name);
            });
        }
    }
}

impl<T> SynchiChannel<T>
where
    T: Any + Clone + Debug + Merge + Default,
{
    pub fn new(name: &'static str) -> Self {
        if let Some(index) = register_channel(name, T::default()) {
            SynchiChannel {
                name,
                index,
                _marker: std::marker::PhantomData,
            }
        } else {
            panic!("Failed to register SYNCHI channel");
        }
    }

    pub fn new_with_data(name: &'static str, data: T) -> Self {
        if let Some(index) = register_channel(name, data) {
            SynchiChannel {
                name,
                index,
                _marker: std::marker::PhantomData,
            }
        } else {
            panic!("Failed to register SYNCHI channel");
        }
    }

    pub fn pull(&self) -> T {
        SYNCHI.with(|store| {
            let store = store.borrow();

            if let Some(channel) = store.get(self.name) {
                let channel = channel.borrow();

                if let Some(data) = channel.get(&self.index) {
                    match data {
                        DataStatus::Free(data) => {
                            let data = data.downcast_ref::<T>().unwrap();
                            data.clone()
                        }
                        DataStatus::Claimed(_, data) => {
                            let data = data.downcast_ref::<T>().unwrap();
                            data.clone()
                        }
                    }
                } else {
                    panic!("Failed to get SYNCHI channel data");
                }
            } else {
                panic!("Failed to get SYNCHI channel");
            }
        })
    }

    pub fn push(&self, data: T) {
        let mut target_subscriber = None::<SynchiSubscriber>;

        SYNCHI.with(|store| {
            let store = store.borrow_mut();

            if let Some(channel) = store.get(self.name) {
                let mut channel = channel.borrow_mut();

                if let Some(channel_data) = channel.get_mut(&self.index) {
                    match channel_data {
                        DataStatus::Free(_) => {
                            *channel_data = DataStatus::Free(Box::new(data.clone()));
                        }
                        DataStatus::Claimed(subscriber, _) => {
                            target_subscriber = subscriber.clone().into();

                            *channel_data =
                                DataStatus::Claimed(subscriber.clone(), Box::new(data.clone()));
                        }
                    }
                } else {
                    panic!("Failed to get SYNCHI channel data");
                }
            } else {
                panic!("Failed to get SYNCHI channel");
            }
        });

        if let Some(subscriber) = target_subscriber {
            notify_subscriber::<T>(self.name, subscriber.clone());
        }
    }
}

#[hook]
pub fn use_synchi_channel<T>(name: &'static str) -> Rc<RefCell<SynchiChannel<T>>>
where
    T: Any + Clone + Default + Merge + Debug,
{
    use_mut_ref(|| SynchiChannel::<T>::new(name))
}

#[hook]
pub fn use_synchi_channel_with<T>(name: &'static str, data: T) -> Rc<RefCell<SynchiChannel<T>>>
where
    T: Any + Clone + Default + PartialEq + Merge + Debug,
{
    use_mut_ref(|| SynchiChannel::<T>::new_with_data(name, data.clone()))
}

#[hook]
pub fn use_synchi_channel_subscribe<T>(name: &'static str, indexes: Vec<usize>) -> UseStateHandle<T>
where
    T: Any + Clone + Default + PartialEq + Debug + Merge,
{
    let data = use_state(|| T::default());

    let subscriber = {
        let data = data.clone();

        use_callback((), move |next_data: Box<dyn Any>, _| {
            if let Some(next_data) = next_data.downcast_ref::<T>() {
                data.set(next_data.clone());
            } else {
                panic!("Failed to downcast SYNCHI channel \"{name}\" data: {next_data:?}");
            }
        })
    };

    subscribe_to_channel::<T>(name, indexes, subscriber.clone());

    use_effect_with((), move |_| {
        let subscriber = subscriber.clone();

        move || {
            unsubscribe_from_channel::<T>(name, subscriber);
        }
    });

    data
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[derive(Debug, Clone, PartialEq, Default)]
    struct MergeInt(i32);

    impl Merge for MergeInt {
        fn merge(&self, other: &Self) -> Self {
            MergeInt(self.0 + other.0)
        }
    }

    #[test]
    #[serial]
    fn test_synchi_channel() {
        let channel = SynchiChannel::<MergeInt>::new("test");
        assert_eq!(channel.index, 0);
    }

    #[test]
    #[serial]
    fn test_several_values_for_synchi_channel() {
        let channel = SynchiChannel::<MergeInt>::new("test");
        assert_eq!(channel.index, 0);

        let channel = SynchiChannel::<MergeInt>::new("test");
        assert_eq!(channel.index, 1);
    }

    #[test]
    #[serial]
    fn test_pull_value_from_synchi_channel() {
        let channel1 = SynchiChannel::<MergeInt>::new("test");
        assert_eq!(channel1.index, 0);

        let channel2 = SynchiChannel::<MergeInt>::new("test");
        assert_eq!(channel2.index, 1);

        let channel3 = SynchiChannel::<MergeInt>::new("test");
        assert_eq!(channel3.index, 2);

        assert_eq!(channel1.pull(), MergeInt(0));
        assert_eq!(channel2.pull(), MergeInt(0));
        assert_eq!(channel3.pull(), MergeInt(0));
    }

    #[test]
    #[serial]
    fn test_push_value_to_synchi_channel() {
        let channel = SynchiChannel::<MergeInt>::new("test");
        assert_eq!(channel.index, 0);

        channel.push(MergeInt(42));
        assert_eq!(channel.pull(), MergeInt(42));
    }

    #[test]
    fn test_push_several_values_to_synchi_channel() {
        let channel = SynchiChannel::<MergeInt>::new("test");
        assert_eq!(channel.index, 0);

        channel.push(MergeInt(42));
        assert_eq!(channel.pull(), MergeInt(42));

        channel.push(MergeInt(43));
        assert_eq!(channel.pull(), MergeInt(43));
    }

    #[test]
    #[serial]
    fn test_push_several_values_from_several_channels_to_synchi_channel() {
        let channel1 = SynchiChannel::<MergeInt>::new("test");
        assert_eq!(channel1.index, 0);

        let channel2 = SynchiChannel::<MergeInt>::new("test");
        assert_eq!(channel2.index, 1);

        let channel3 = SynchiChannel::<MergeInt>::new("test");
        assert_eq!(channel3.index, 2);

        channel1.push(MergeInt(42));
        assert_eq!(channel1.pull(), MergeInt(42));

        channel2.push(MergeInt(43));
        assert_eq!(channel2.pull(), MergeInt(43));

        channel3.push(MergeInt(44));
        assert_eq!(channel3.pull(), MergeInt(44));

        // Check that the values are still there
        assert_eq!(channel1.pull(), MergeInt(42));
        assert_eq!(channel2.pull(), MergeInt(43));
        assert_eq!(channel3.pull(), MergeInt(44));
    }

    #[test]
    #[serial]
    fn test_if_one_subscriber_is_notified() {
        let channel = SynchiChannel::<MergeInt>::new("test");
        assert_eq!(channel.index, 0);

        let received_data = Rc::new(RefCell::new(Vec::new()));

        let subscriber = {
            let received_data = received_data.clone();

            Callback::from(move |data: Box<dyn Any>| {
                if let Some(data) = data.downcast_ref::<MergeInt>() {
                    received_data.borrow_mut().push(data.clone());
                } else {
                    panic!("Failed to downcast SYNCHI channel data");
                }
            })
        };

        // Subscribed before any data is pushed
        subscribe_to_channel::<MergeInt>("test", vec![0], subscriber.clone());

        // The subscriber should receive the initial value
        assert_eq!(received_data.borrow().len(), 1);
        assert_eq!(received_data.borrow()[0], MergeInt(0));

        channel.push(MergeInt(42));
        assert_eq!(channel.pull(), MergeInt(42));
        assert_eq!(received_data.borrow().len(), 2);
        assert_eq!(received_data.borrow()[1], MergeInt(42));

        channel.push(MergeInt(43));
        assert_eq!(channel.pull(), MergeInt(43));
        assert_eq!(received_data.borrow().len(), 3);
        assert_eq!(received_data.borrow()[2], MergeInt(43));

        unsubscribe_from_channel::<MergeInt>("test", subscriber);

        // No new data should be received
        channel.push(MergeInt(44));
        assert_eq!(channel.pull(), MergeInt(44));
        assert_eq!(received_data.borrow().len(), 3);
    }

    #[test]
    #[serial]
    fn test_several_subscribers_for_one_channel() {
        let channel = SynchiChannel::<MergeInt>::new("test");
        assert_eq!(channel.index, 0);

        let received_data_1 = Rc::new(RefCell::new(Vec::new()));

        let subscriber_1 = {
            let received_data = received_data_1.clone();

            Callback::from(move |data: Box<dyn Any>| {
                if let Some(data) = data.downcast_ref::<MergeInt>() {
                    received_data.borrow_mut().push(data.clone());
                } else {
                    panic!("Failed to downcast SYNCHI channel data");
                }
            })
        };

        let received_data_2 = Rc::new(RefCell::new(Vec::new()));

        let subscriber_2 = {
            let received_data = received_data_2.clone();

            Callback::from(move |data: Box<dyn Any>| {
                if let Some(data) = data.downcast_ref::<MergeInt>() {
                    received_data.borrow_mut().push(data.clone());
                } else {
                    panic!("Failed to downcast SYNCHI channel data");
                }
            })
        };

        // Subscribe each after another
        subscribe_to_channel::<MergeInt>("test", vec![0], subscriber_1.clone());
        subscribe_to_channel::<MergeInt>("test", vec![1], subscriber_2.clone());

        // Both subscribers should receive the initial value
        assert_eq!(received_data_1.borrow().len(), 1);
        assert_eq!(received_data_1.borrow()[0], MergeInt(0));
        assert_eq!(received_data_2.borrow().len(), 1);
        assert_eq!(received_data_2.borrow()[0], MergeInt(0));

        // And the first subscriber should receive the new data
        channel.push(MergeInt(42));
        assert_eq!(channel.pull(), MergeInt(42));
        assert_eq!(received_data_1.borrow().len(), 2);
        assert_eq!(received_data_1.borrow()[1], MergeInt(42));
        assert_eq!(received_data_2.borrow().len(), 1);

        // Free the first subscriber
        unsubscribe_from_channel::<MergeInt>("test", subscriber_1);

        // No new data should be received
        channel.push(MergeInt(44));
        assert_eq!(channel.pull(), MergeInt(44));
        assert_eq!(received_data_1.borrow().len(), 2);

        // The second subscriber do not receive the new data bc it wasn't claimed by it
        assert_eq!(received_data_2.borrow().len(), 1);

        // But the new subscriber should receive the new data
        let received_data_3 = Rc::new(RefCell::new(Vec::new()));

        let subscriber_3 = {
            let received_data = received_data_3.clone();

            Callback::from(move |data: Box<dyn Any>| {
                if let Some(data) = data.downcast_ref::<MergeInt>() {
                    received_data.borrow_mut().push(data.clone());
                } else {
                    panic!("Failed to downcast SYNCHI channel data");
                }
            })
        };

        subscribe_to_channel::<MergeInt>("test", vec![0], subscriber_3.clone());

        assert_eq!(received_data_3.borrow().len(), 1);
        assert_eq!(received_data_3.borrow()[0], MergeInt(44));

        // And the second subscriber should still have the initial value
        assert_eq!(received_data_2.borrow().len(), 1);
        assert_eq!(received_data_2.borrow()[0], MergeInt(0));
    }

    #[test]
    #[serial]
    fn test_one_subscriber_for_multiple_subchannels() {
        let channel_1 = SynchiChannel::<MergeInt>::new("test");
        let channel_2 = SynchiChannel::<MergeInt>::new("test");

        let received_data = Rc::new(RefCell::new(Vec::new()));

        let subscriber = {
            let received_data = received_data.clone();

            Callback::from(move |data: Box<dyn Any>| {
                if let Some(data) = data.downcast_ref::<MergeInt>() {
                    received_data.borrow_mut().push(data.clone());
                } else {
                    panic!("Failed to downcast SYNCHI channel data");
                }
            })
        };

        subscribe_to_channel::<MergeInt>("test", vec![0, 1], subscriber.clone());

        // Since the data will merged before being sent to the subscriber, the subscriber should
        // receive the initial value
        assert_eq!(received_data.borrow().len(), 1);
        assert_eq!(received_data.borrow()[0], MergeInt(0));

        channel_1.push(MergeInt(42));

        assert_eq!(channel_1.pull(), MergeInt(42));
        assert_eq!(received_data.borrow().len(), 2);
        assert_eq!(received_data.borrow()[1], MergeInt(42));

        channel_2.push(MergeInt(43));

        assert_eq!(channel_2.pull(), MergeInt(43));
        assert_eq!(received_data.borrow().len(), 3);
        assert_eq!(received_data.borrow()[2], MergeInt(85));
    }

    #[test]
    #[serial]
    fn test_several_subscribers_for_several_channels() {
        let channel_1 = SynchiChannel::<MergeInt>::new("test");
        channel_1.push(MergeInt(42));
        let received_data_1 = Rc::new(RefCell::new(Vec::new()));

        let subscriber_1 = {
            let received_data = received_data_1.clone();

            Callback::from(move |data: Box<dyn Any>| {
                if let Some(data) = data.downcast_ref::<MergeInt>() {
                    received_data.borrow_mut().push(data.clone());
                } else {
                    panic!("Failed to downcast SYNCHI channel data");
                }
            })
        };

        subscribe_to_channel::<MergeInt>("test", vec![0], subscriber_1.clone());
        assert!(received_data_1.borrow().len() == 1);
        assert_eq!(received_data_1.borrow()[0], MergeInt(42));

        let channel_2 = SynchiChannel::<MergeInt>::new("test");
        channel_2.push(MergeInt(43));
        let received_data_2 = Rc::new(RefCell::new(Vec::new()));

        let subscriber_2 = {
            let received_data = received_data_2.clone();

            Callback::from(move |data: Box<dyn Any>| {
                if let Some(data) = data.downcast_ref::<MergeInt>() {
                    received_data.borrow_mut().push(data.clone());
                } else {
                    panic!("Failed to downcast SYNCHI channel data");
                }
            })
        };

        subscribe_to_channel::<MergeInt>("test", vec![1], subscriber_2.clone());
        // The first subscriber still has a prev value from the first channel
        assert_eq!(received_data_1.borrow().len(), 1);
        assert_eq!(received_data_1.borrow()[0], MergeInt(42));

        // The second subscriber should receive it's own value from the second channel
        assert_eq!(received_data_2.borrow().len(), 1);
        assert_eq!(received_data_2.borrow()[0], MergeInt(43));
    }
}
