pub trait Strategy<Config> {
    fn name(&self) -> &str;
}

// fn create<Config>(config: Config) -> impl Strategy<Config> {
//     todo!()
// }
