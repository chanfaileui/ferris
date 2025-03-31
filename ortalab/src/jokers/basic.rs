// src/jokers/basic.rs
use crate::errors::GameResult;
use crate::game::GameState;
use crate::jokers::ActivationType;
use crate::jokers::JokerEffect;
use ortalib::JokerCard;

// ✖ Mult +4
pub struct Joker;

impl JokerEffect for Joker {}

// ✖ Mult +8 if cards played contains a Pair
pub struct JollyJoker;

impl JokerEffect for JollyJoker {}

// ✖ Mult +12 if cards played contains a Three of a Kind
pub struct ZanyJoker;

impl JokerEffect for ZanyJoker {}

// ✖ Mult +10 if cards played contains a Two Pair
pub struct MadJoker;

impl JokerEffect for MadJoker {}

// ✖ Mult +12 if cards played contains a Straight
pub struct CrazyJoker;

impl JokerEffect for CrazyJoker {}

// ✖ Mult +10 if cards played contains a Flush
pub struct DrollJoker;

impl JokerEffect for DrollJoker {}

// 🪙 Chips +50 if cards played contains a Pair
pub struct SlyJoker;

impl JokerEffect for SlyJoker {}

// 🪙 Chips +100 if cards played contains a Three of a Kind
pub struct WilyJoker;

impl JokerEffect for WilyJoker {}

// 🪙 Chips +80 if cards played contains a Two Pair
pub struct CleverJoker;

impl JokerEffect for CleverJoker {}

// 🪙 Chips +100 if cards played contains a Straight
pub struct DeviousJoker;

impl JokerEffect for DeviousJoker {}

// 🪙 Chips +80 if cards played contains a Flush
pub struct CraftyJoker;

impl JokerEffect for CraftyJoker {}

// ✖ Mult +3 for each Joker card
pub struct AbstractJoker;

impl JokerEffect for AbstractJoker {}
