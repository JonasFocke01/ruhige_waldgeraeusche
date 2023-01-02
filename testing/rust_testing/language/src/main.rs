enum InputType {
    Serial,
    RestApi
}

fn main(){
    let input_type = InputType::RestApi;
    // let input_type = InputType::RestApi;

    match input_type {
        InputType::Serial => print!("Serial\n"),
        InputType::RestApi => print!("RestApi\n")
    };
}