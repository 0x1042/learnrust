use tracing::info;

pub trait Animal {
    // Self 具体实现的返回
    fn new(name: String) -> Self;

    fn name(&self) -> String;

    fn nosie(&self) -> String;

    fn take(&self) {
        info!("default implement for {} -> {}", self.name(), self.nosie());
    }
}

pub struct Sheep {
    naked: bool,
    name: String,
}

impl Animal for Sheep {
    fn new(name: String) -> Self {
        Sheep { naked: false, name }
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn nosie(&self) -> String {
        if self.naked {
            "!!!!".to_owned()
        } else {
            "????".to_owned()
        }
    }

    fn take(&self) {
        info!("sheep implement for {} -> {}", self.name(), self.nosie());
    }
}

pub struct Dog {
    name: String,
}

impl Animal for Dog {
    fn new(name: String) -> Self {
        Dog { name }
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn nosie(&self) -> String {
        "wangwangwang!!!".to_owned()
    }

    fn take(&self) {
        info!("dog implement for {} -> {}", self.name(), self.nosie());
    }
}

pub fn test_trait() {
    let mut dolly: Sheep = Animal::new("dolly".to_owned());
    dolly.take();
    let mut wangcai: Dog = Animal::new("wangcai".to_string());
    wangcai.take();
}
