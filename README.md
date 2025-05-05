# Description

This program will download the [listed mods](#mods).

Will store which was the latest version download and in case of detecting a new version, download the new version.

Some mods allow to be automatically patched, although it doesn't check if the game is already patched or not.

The mods will be downloaded where this script executable is located.

A file named db.json will be used to keep track of the repos, their settings and their latest version applied.

```text
# tree   
.
├── db.json
├── Iquis
│   └── rev2-wakeup-tool
├── kkots
│   ├── GGXrdBackgroundGamepad
│   ├── GGXrdFasterLoadingTimes
│   ├── ggxrd_hitbox_overlay_2211
│   ├── GGXrdMirrorColorSelect
│   └── rev2-wakeup-tool
└── xrd_mods_downloader.exe
```

## Mods

| mod                                                                                   | can be patched (Linux) | can be patched (Windows) |
|---------------------------------------------------------------------------------------|------------------------|--------------------------|
| [Iquis/rev2-wakeup-tool](https://github.com/Iquis/rev2-wakeup-tool)                   |                        |                          |
| [kkots/rev2-wakeup-tool](https://github.com/kkots/rev2-wakeup-tool)                   |                        |                          |
| [kkots/ggxrd_hitbox_overlay_2211](https://github.com/kkots/ggxrd_hitbox_overlay_2211) | ✅                      | ✅? Test pending          |
| [kkots/GGXrdFasterLoadingTimes](https://github.com/kkots/GGXrdFasterLoadingTimes)     | ✅                      | ✅? Test pending          |
| [kkots/GGXrdMirrorColorSelect](https://github.com/kkots/GGXrdMirrorColorSelect)       |                        |                          |
| [kkots/GGXrdBackgroundGamepad](https://github.com/kkots/GGXrdBackgroundGamepad)       | ✅                      | ✅? Test pending          |

## How to Use

## Enable Patching

Set the value `automatically_patch` to `true` from the app to patch in the file `db.json`.

Since the file generated doesn't display a friendly format, pages such as https://jsonviewer.stack.hu/ can be used to format the json and afterward modify it. 

Once modified, the next time the script is used the mod will be patched.

```json
{
    ...
    "kkots/GGXrdBackgroundGamepad": {
      "repo_owner": "kkots",
      "repo_name": "GGXrdBackgroundGamepad",
      "app_type": "BackgroundGamepad",
      "id": 205349779,
      "tag_name": "1",
      "published_at": "2025-03-12T16:23:27Z",
      "url_source_version": "https://github.com/kkots/GGXrdBackgroundGamepad/releases/tag/1",
      "automatically_patch": false,  ->  true
      "patched": false
    }
    ...
}
```

### Note

If the binary of GuiltyGearXrd.exe file was previously patched, it won't be detected and patched all over again.

As well, if there is a new versions of a mod it won't be automatically patched.

**The patching is done through the respective files provided by the mod.**

