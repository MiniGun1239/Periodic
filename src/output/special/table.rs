use colored::*;

pub fn normal_colored() {
    let s = |txt: &str| txt.red().bold();
    let d = |txt: &str| txt.yellow();
    let p = |txt: &str| txt.green();
    let f = |txt: &str| txt.magenta();
    let grid = |txt: &str| txt.dimmed(); // Minimal gray borders

    // 2. Print line by line
    println!("  {}                                                                    {}", s("H "), s("He"));
    println!("  {       }                                              {               }", grid("[s-block]"), grid("[--- p-block ---]"));
    println!("  {}     {}                                              {} {} {} {} {} {}", s("Li"), s("Be"), p("B "), p("C "), p("N "), p("O "), p("F "), p("Ne"));
    println!("  {}     {}             {                           }    {} {} {} {} {} {}", s("Na"), s("Mg"), grid("[--------- d-block ---------]"), p("Al"), p("Si"), p("P "), p("S "), p("Cl"), p("Ar"));
    println!("  {}     {}             {} {} {} {} {} {} {} {} {} {}    {} {} {} {} {} {}", s("K "), s("Ca"), d("Sc"), d("Ti"), d("V "), d("Cr"), d("Mn"), d("Fe"), d("Co"), d("Ni"), d("Cu"), d("Zn"), p("Ga"), p("Ge"), p("As"), p("Se"), p("Br"), p("Kr"));
    println!("  {}     {}             {} {} {} {} {} {} {} {} {} {}    {} {} {} {} {} {}", s("Rb"), s("Sr"), d("Y "), d("Zr"), d("Nb"), d("Mo"), d("Tc"), d("Ru"), d("Rh"), d("Pd"), d("Ag"), d("Cd"), p("In"), p("Sn"), p("Sb"), p("Te"), p("I "), p("Xe"));
    println!("  {}     {}             {} {} {} {} {} {} {} {} {} {}    {} {} {} {} {} {}", s("Cs"), s("Ba"), f("* "), d("Hf"), d("Ta"), d("W "), d("Re"), d("Os"), d("Ir"), d("Pt"), d("Au"), d("Hg"), p("Tl"), p("Pb"), p("Bi"), p("Po"), p("At"), p("Rn"));
    println!("  {}     {}             {} {} {} {} {} {} {} {} {} {}    {} {} {} {} {} {}", s("Fr"), s("Ra"), f("# "), d("Rf"), d("Db"), d("Sg"), d("Bh"), d("Hs"), d("Mt"), d("Ds"), d("Rg"), d("Cn"), p("Nh"), p("Fl"), p("Mc"), p("Lv"), p("Ts"), p("Og"));
    println!();
    println!("  { }    {                }", s("Uue"), grid("] --- hypothetical"));
    println!();
    println!("                         {                                       }", grid("[--------------- f-block ---------------]"));
    println!("           {           } {} {} {} {} {} {} {} {} {} {} {} {} {} {}", f("*Lanthanides:"), f("Ce"), f("Pr"), f("Nd"), f("Pm"), f("Sm"), f("Eu"), f("Gd"), f("Tb"), f("Dy"), f("Ho"), f("Er"), f("Tm"), f("Yb"), f("Lu"));
    println!("           {           } {} {} {} {} {} {} {} {} {} {} {} {} {} {}", f("#Actinides:  "), f("Th"), f("Pa"), f("U "), f("Np"), f("Pu"), f("Am"), f("Cm"), f("Bk"), f("Cf"), f("Es"), f("Fm"), f("Md"), f("No"), f("Lr"));
    println!("---")
}
