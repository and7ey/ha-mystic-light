# Mystic Light Server Integration with Home Assistant

Home Assistant requires both the installed [Mystic Light](https://www.msi.com/Landing/mystic-light-rgb-gaming-pc/mystic-light) software and this application on Windows to function properly.

## Device Compatibility Check

To determine if your device is supported:

- Verify your device specifications for any mention of "Mystic Light"
- Consult the official [Mystic Light compatible products](https://www.msi.com/Landing/mystic-light-rgb-gaming-pc/compatible-products) list
- If you have installed "Dragon Center" or "MSI Center," check the Mystic Light tab for supported devices

## Setup Instructions

1. Ensure the "MSI SDK" program is installed on your Windows PC. This typically comes bundled with either "Dragon Center" or "MSI Center." Download links can be found [here](https://www.msi.com/Landing/mystic-light-rgb-gaming-pc/download).
2. Install this application
3. Install the Home Assistant custom component

### Application Installation Steps

1. Visit the [releases page](https://github.com/and7ey/mystic-light-ha-app/releases)
2. Locate the latest release
3. Find and click the download link labeled "installer.zip"
4. Run the downloaded installer to install the app (Windows Defender may flag this as potentially malicious; add the file to the exclusion list if necessary)
5. Add the application executable (service.exe) to the list of allowed apps through Windows Defender Firewall settings (refer to this [instruction](https://support.microsoft.com/en-us/windows/risks-of-allowing-apps-through-windows-defender-firewall-654559af-3f54-3dcf-349f-71ccd90bcc5c))

To verify successful installation:

1. Press `Win+R`, type `services.msc`, and press Enter
2. Locate the "Mystic Light HA" service
3. Confirm that the service status shows as "Running"

## Troubleshooting

Looks at `C:\ProgramData\Mystic Light\Mystic Light HA` for the logs.
