

use dioxus::prelude::*;
use serde_json::json;



/// Tab component for switching between keyboard views
#[component]
pub fn KeyboardAssumptions() -> Element {


    rsx! {
  //The proper keyboard spacing, also known as pitch, is generally around 19mm (0.75 inches) between the centers of the keys. This standard spacing is used on most full-size and many smaller keyboards. While some variation exists, especially in smaller or specialized keyboards, the 19mm standard is a good starting point for ergonomic typing. 


// spacing
//keycap spacing


//Cherry MX key dimensions, specifically referring to the switch stem, are typically 4.1mm x 4.1mm with arms that are 1.17mm wide. The total height of the stem is around 4.1mm. However, some users have measured the stem slightly differently, with a width of 3.89mm, a height of 3.69mm, and a thickness of 1.3mm on one side and 1.1mm


p{"1. Keycap Sizes (Relative Units)

Most keyboards use 1u (1 unit) as the base size (typically ~19mm square), with other sizes being multiples:

1u = Base size (19×19mm) - Alphanumeric keys
1.25u = 23.75mm - Common modifier keys (Tab, Ctrl)
1.5u = 28.5mm - Caps Lock, some Enter keys
1.75u = 33.25mm - Right Shift (60% keyboards)
2u = 38mm - Spacebars, numpad "0" (often stabilized)
2.25u = 42.75mm - Common left Shift size
2.75u = 52.25mm - Some right Shift keys
6.25u = ~118.75mm - Standard full-size spacebar
7u = ~133mm - Compact/TKL spacebar
2. Keycap Height & Profile Assumptions

Row heights vary (R1-R5 in Cherry profile, R1-R4 in SA/DSA)
Common profiles:
Cherry/OEM: Sculpted (different heights per row)
DSA/XDA: Uniform spherical (same height)
SA: High-profile, spherical
3. Keyboard Layout Assumptions

ANSI (US) Layout:
Enter = 2.25u × 1u (vertical rectangle)
Left Shift = 2.25u
Right Shift = 1.25u or 2.75u
Backspace = 2u
ISO (EU) Layout:
Enter = 1.25u × 2u (horizontal rectangle)
Left Shift = 1.25u
Right Shift = 1.75u or 2.75u
4. Stabilizer Assumptions

Required for keys ≥2u (Spacebar, Enter, Shift, etc.)
Common stabilizer sizes:
2u (Numpad "+", "0")
6.25u / 7u (Spacebar)
5. Default Spacing & Alignment

Gap between keys: ~0.25u (~4.75mm)
Key alignment:
Centered legends by default
Top-left legends for secondary functions (e.g., Fn layer)"}

    }
}

