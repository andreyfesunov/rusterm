fn to_abstract_socket(host: &String, port: &u16) -> String {
    format!("\0rusterm-sock-{}:{}", host, port)
}
