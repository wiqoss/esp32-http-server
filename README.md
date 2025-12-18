It creates a Wi-Fi network (without Internet connection, of course) with those credentials: <br>
```
SSID: ESP32
Password: 13201930
```
You can also change credentials in src/romaing.rs
<br>
<br>
<h3>How to build and run</h3>
Install https://github.com/esp-rs/esp-idf-svc and run the command:
```
cargo espflash flash --monitor
```
<br>
<br>
<h3>Current routes</h3>
For me LAN IP-Address is 192.168.71.1, I'm going to use it here.
Default port is 80 (HTTP Default port).

[http://192.168.71.1/switch_light](http://your_esp_lan_address/switch_light) - toggle blue LED blinking