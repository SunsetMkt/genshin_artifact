use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum CharacterConfig {
    Ganyu { talent2_rate: f64 },
    HuTao { le_50: bool },
    KamisatoAyaka { talent1_rate: f64, talent2_rate: f64 },
    Keqing { talent2_rate: f64 },
    KukiShinobu { hp_le_50: bool, use_c6: bool },
    Ningguang { talent2_rate: f64 },
    Rosaria { e_from_behind: bool },
    Razor { e_stack: f64, talent2_ratio: f64 },
    Yelan { team_element_count: usize },
    Yoimiya { talent1_level: f64 },
    Collei { background: bool },
    Tighnari { talent1_ratio: f64, c2_ratio: f64 },
    Cyno { c2_stack: f64, after_q: bool },
    Nilou { golden_rate: f64 },
    Candace { c2_rate: f64 },
    Nahida { c4_e_count: usize },
    Wanderer { e_pyro: bool, e_cryo: bool },
    Faruzan { q_ratio: f64 },
    Yaoyao { c4_rate: f64 },
    Alhaitham { c2_stack: f64, c4_stack: f64, c6_rate: f64 },
    Kaveh { talent2_stack: f64, c2_rate: f64 },
    Baizhu { hp_below_50: bool },
    Lynette { talent1_rate: f64, talent1_count: usize, talent2_rate: f64 },
    Freminet { c4_stack: f64, c6_stack: f64 },
    Lyney { c2_stack: f64, c4_rate: f64 },
    Neuvillette { current_hp: usize },
    Wriothesley { talent2_stack: f64 },
    Charlotte { talent2_fontaine_count: usize, talent2_non_fontaine_count: usize, c2_count: usize, c2_rate: f64 },
    Furina { c2_overflow: f64 },
    Chevreuse { talent1_rate: f64, talent2_rate: f64, c6_stack: f64 },
    Navia { talent2_character_count: usize },
    Gaming { hp_above50: bool, c2_rate: f64 },
    Xianyun { talent1_stack: f64, talent2_rate: f64, butianti_count: usize },
    Chiori { talent2: bool },
    Arlecchino { c6_ratio: f64 },
    Clorinde { talent1_stack: f64, talent2_stack: f64, c6_rate: f64 },
    Sigewinne { c6_rate: f64 },
    Sethos { c2_stack: f64 },
    NoConfig,
}
