WZ Cli Tool
===========

Tool for extracting and building WZ archives and images.

## WZ Archives

Denoted with `.wz` extension. These files contain binary files \(`images`\) organized in a directory-like structure.

The arguments are similar to `tar`.

Extracting a WZ archive:

```bash no_build
wzarchive -m 83 -k gms -xvf Character.wz
wzarchive -m 176 -xvf Character.wz
```

Building a WZ archive:

```bash no_build
wzarchive -m 83 -k gms -cvf Character.wz ./Character/
wzarchive -m 176 -cvf Character.wz ./Character/
```

Writing Server XML files

```bash no_build
wzarchive -m 83 -k gms -Svf Character.wz
wzarchive -m 176 -Svf Character.wz
```

## WZ Images

Binary packages found within WZ archives. Can be extracted again after using `wzarchive`.

Extracting a WZ image:

```bash no_build
wzimage -k gms -xvf Character/Weapon/01472030.img
wzimage -xvf Character/Weapon/01472030.img
```

Building a WZ image:

```bash no_build
wzimage -k gms -cvf 01472030.img 01472030/01472030.img.xml
wzimage -cvf 01472030.img 01472030/01472030.img.xml
```
