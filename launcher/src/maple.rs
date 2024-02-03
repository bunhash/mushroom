//! Hook MapleStory.exe locations
//!
//! These instructions open the HTML launcher:
//!
//! ...
//! 009F18C9 | 55                       | push ebp |
//! 009F18CA | 8BEC                     | mov ebp,esp |
//! 009F18CC | 81EC 10020000            | sub esp,210 |
//! 009F18D2 | 80A5 F0FDFFFF 00         | and byte ptr ss:[ebp-210],0 |
//! 009F18D9 | 8D85 F0FDFFFF            | lea eax,dword ptr ss:[ebp-210] |
//! 009F18DF | 68 B8F2B300              | push maplestory.B3F2B8 | B3F2B8:"http://Ingameweb.nexon.net/maplestory/client/launcher.html"
//! 009F18E4 | 50                       | push eax | eax:&"WvsClientMtx"
//! 009F18E5 | E8 660C0700              | call maplestory.A62550 |
//! 009F18EA | 59                       | pop ecx |
//! 009F18EB | 59                       | pop ecx |
//! 009F18EC | 6A 00                    | push 0 |
//! 009F18EE | C745 F0 76020000         | mov dword ptr ss:[ebp-10],276 |
//! 009F18F5 | C745 F4 94020000         | mov dword ptr ss:[ebp-C],294 |
//! 009F18FC | C745 F8 05000000         | mov dword ptr ss:[ebp-8],5 |
//! 009F1903 | FF15 8802BF00            | call dword ptr ds:[<&GetModuleHandleA>] |
//! 009F1909 | 8945 FC                  | mov dword ptr ss:[ebp-4],eax |
//! 009F190C | 8D85 F0FDFFFF            | lea eax,dword ptr ss:[ebp-210] |
//! 009F1912 | 50                       | push eax | eax:&"WvsClientMtx"
//! 009F1913 | E8 5892D8FF              | call maplestory.77AB70 |
//! 009F1918 | 59                       | pop ecx |
//! 009F1919 | C9                       | leave |
//! 009F191A | C3                       | ret |
//! ...
//! 009F1C04 | E8 C0FCFFFF              | call maplestory.9F18C9 |
//! ...
//!
//! This can probably be hijacked for my initial codecave.
