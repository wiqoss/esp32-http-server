### What does it do

It creates a Wi-Fi network (without Internet connection, of course) with those credentials:

```
SSID: ESP-32
Password: 12345678
```
You can also change credentials in src/romaing.rs

### How to build and run
Install [esp-idf-svc](https://github.com/esp-rs/esp-idf-svc) and run the command:

```
cargo espflash flash --monitor
```

### Current routes
For me LAN IP-Address is 192.168.71.1, I'm going to use it here.
Default port is 80 (HTTP Default port).

[http://192.168.71.1/](http://your_esp_lan_address/) - just index page

[http://192.168.71.1/switch_light](http://your_esp_lan_address/switch_light) - toggle blue LED blinking
