# Setup WSL (Windows 11)
## 1. Instalar dependencias
Abrir consola WSL y ejecutar el siguiente comando
```bash
sudo apt install git pkg-config libfprint-2-2 libfprint-2-dev libglib2.0-dev libclang-dev cargo linux-tools-generic hwdata
```

## 2. Instalar dependencias para [conectar usb a WSL](https://learn.microsoft.com/en-us/windows/wsl/connect-usb)
### 2.1 Instalar [USBIPD-WIN](https://github.com/dorssel/usbipd-win)
* Ir al [repositorio oficial](https://github.com/dorssel/usbipd-win/releases/) de USBIPD-WIN.
* Seleccionar el archivo .msi lo cual descargara el instalador
* Ejecutar el archivo .msi descargado
### 2.2 Instalar las herramientas USBIP y la base de datos de hardware de Linux en WSL
Abrir una consola de WSL (Ubuntu) y ejecutar el siguiente comando: 
```bash
sudo update-alternatives --install /usr/local/bin/usbip usbip /usr/lib/linux-tools/*-generic/usbip 20
```
### 2.3 Conectar el dispositivo usb a Linux
Enlista tus dispositivos usb disponibles en PowerShell ejecutando el siguiente comando: 
```PowerShell
usbipd wsl list
```
Selecciona el bus id que corresponda al "U.are.U® 4500 Fingerprint Reader" y ejecuta el siguiente comando en PowerShell:
```bash
usbipd wsl attach --busid <busid>
```
En la consola de WSL, ejecuta el siguiente comando:
```
lsusb
```
Deberias ver el disposito usb en la lista de dispositivos conectados.
## 3. Compilar el servidor socket
En wsl, descarga el repositorio: 
```
git clone https://github.com/AlvaroParker/fingerprint-rs.git
```
Entra al directorio del respositorio y ejecuta el commando `cargo build`, este puede tomar un tiempo:
```
cd fingerprint-rs/
cargo build
```
Luego ejecuta `cargo run`
```
cargo run
```
La primera vez que se ejecuta, es posible que se presente el siguiente mensaje de erorr:
```
libusb: error [get_usbfs_fd] libusb couldn't open USB device /dev/bus/usb/001/002, errno=13
libusb: error [get_usbfs_fd] libusb requires write access to USB device nodes
USB error on device 05ba:000a : Access denied (insufficient permissions) [-3]
```
Para arreglar esto:
```
sudo chown <tu-usuario-de-wsl> /dev/bus/usb/001/002
```
Donde `/dev/bus/usb/001/002` el dispositivo que te haya salido en el mensaje de error.
Finalmente, corre `cargo run` nuevamente:
```
cargo run
```
El siguiente mensaje deberia aparecer y el servido socket deberia estar corriendo:
```
Binding to <dirrecion ip>
```
