
pub fn init_logger(file_name: &str){
    log4rs::init_file(file_name, Default::default()).unwrap()
}