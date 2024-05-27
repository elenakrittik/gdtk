#[derive(Default)]
pub struct Tap(Vec<Arg>);

impl Tap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn arg(mut self, arg: Arg) -> Self {
        self.0.push(arg);
        self
    }
}

pub struct Arg {
    pub short: Option<char>,
    pub long: Option<String>,
    pub required: bool,
    pub description: Option<String>,
    pub type_: Box<dyn std::any::Any>,
}

impl Arg {
    pub fn new<T: Default + 'static>() -> Self {
        Self {
            short: None,
            long: None,
            required: false,
            description: None,
            type_: Box::new(T::default()),
        }
    }

    pub fn short(mut self, short: char) -> Self {
        self.short = Some(short);
        self
    }

    pub fn long(mut self, long: String) -> Self {
        self.long = Some(long);
        self
    }

    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }
}
