# Mystic Light server to work with Home Assistant

> Home Assistant will work only with installed [Mystic Light](https://www.msi.com/Landing/mystic-light-rgb-gaming-pc/mystic-light) and this app on Windows

## Is my device supported?

- check your device info - if there any mention of "Mystic Light" then most likely it is supported
- you can check the [Mystic Light compatible products](https://www.msi.com/Landing/mystic-light-rgb-gaming-pc/compatible-products)
- if you have installed "Dragon Center" or "MSI Center" app you will see your device on the Mystic Light tab if it is supported

## Setup

1. Make sure you have installed "MSI SDK" program installed on your Windows PC. This program is installed with either "Dragon Center" or "MSI Center" app - you can find download links [here](https://www.msi.com/Landing/mystic-light-rgb-gaming-pc/download).
2. Install this app
3. Install [browser extension](#browser-extension)

### The app

1. Go to [releases page](https://github.com/meskill/mystic-light-browser-cinema/releases)
2. Find latest release
3. Locate download link with `Native-app installer` and click on it
4. After download run it to install native app

To verify that everything is set correctly:
1. Press `Win + R`, type `services.msc`, press enter
2. Locate `Mystic Light Browser Cinema` service
3. If it shows state `Running` then native-app is working

<p align="center">
  <img align="center" height="280" src="./assets/running-service.png">
</p>
