use actix_web_static_files::resource_dir;

fn main() {
    resource_dir("/home/kali/Desktop/Shield_website/Website_sf/src/css").build().unwrap();
    resource_dir("/home/kali/Desktop/Shield_website/Website_sf/src/assets").build().unwrap();
    resource_dir("/home/kali/Desktop/Shield_website/Website_sf/src/js").build().unwrap();
}