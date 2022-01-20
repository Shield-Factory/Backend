use actix_web_static_files::resource_dir;

fn main() {
    resource_dir("/home/caluri0/Documents/website_sf/src/css").build().unwrap();
    resource_dir("/home/caluri0/Documents/website_sf/src/assets").build().unwrap();
    resource_dir("/home/caluri0/Documents/website_sf/src/js").build().unwrap();
}