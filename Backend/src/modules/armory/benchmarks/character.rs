extern crate test;
extern crate time;

use test::Bencher;

use mysql_connection::tools::Execute;

use crate::modules::armory::Armory;
use crate::modules::armory::dto::{CharacterDto, CharacterGearDto, CharacterHistoryDto, CharacterInfoDto, CharacterItemDto, GuildDto};
use crate::modules::armory::tools::{SetCharacter};
use self::time::PreciseTime;

/*
* Goal of this benchmark is to see how many
* set operations the server can potentially handle
* incoming from extern servers
*/
#[bench]
fn set_character(_: &mut Bencher) {
    let armory = Armory::default();
    let character_info_dto = CharacterInfoDto {
        gear: CharacterGearDto {
            head: Some(CharacterItemDto {
                item_id: 40427,
                random_property_id: None,
                enchant_id: None,
                gem_ids: vec![None, None, None, None],
            }),
            neck: Some(CharacterItemDto {
                item_id: 39310,
                random_property_id: None,
                enchant_id: None,
                gem_ids: vec![None, None, None, None],
            }),
            shoulder: Some(CharacterItemDto {
                item_id: 39415,
                random_property_id: None,
                enchant_id: None,
                gem_ids: vec![None, None, None, None],
            }),
            back: Some(CharacterItemDto {
                item_id: 40526,
                random_property_id: None,
                enchant_id: None,
                gem_ids: vec![None, None, None, None],
            }),
            chest: Some(CharacterItemDto {
                item_id: 41907,
                random_property_id: None,
                enchant_id: None,
                gem_ids: vec![None, None, None, None],
            }),
            shirt: None,
            tabard: None,
            wrist: Some(CharacterItemDto {
                item_id: 39271,
                random_property_id: None,
                enchant_id: None,
                gem_ids: vec![None, None, None, None],
            }),
            main_hand: Some(CharacterItemDto {
                item_id: 40698,
                random_property_id: None,
                enchant_id: None,
                gem_ids: vec![None, None, None, None],
            }),
            off_hand: Some(CharacterItemDto {
                item_id: 37038,
                random_property_id: None,
                enchant_id: None,
                gem_ids: vec![None, None, None, None],
            }),
            ternary_hand: Some(CharacterItemDto {
                item_id: 39192,
                random_property_id: None,
                enchant_id: None,
                gem_ids: vec![None, None, None, None],
            }),
            glove: Some(CharacterItemDto {
                item_id: 39735,
                random_property_id: None,
                enchant_id: None,
                gem_ids: vec![None, None, None, None],
            }),
            belt: Some(CharacterItemDto {
                item_id: 37189,
                random_property_id: None,
                enchant_id: None,
                gem_ids: vec![None, None, None, None],
            }),
            leg: Some(CharacterItemDto {
                item_id: 39723,
                random_property_id: None,
                enchant_id: None,
                gem_ids: vec![None, None, None, None],
            }),
            boot: Some(CharacterItemDto {
                item_id: 42110,
                random_property_id: None,
                enchant_id: None,
                gem_ids: vec![None, None, None, None],
            }),
            ring1: Some(CharacterItemDto {
                item_id: 37232,
                random_property_id: None,
                enchant_id: None,
                gem_ids: vec![None, None, None, None],
            }),
            ring2: Some(CharacterItemDto {
                item_id: 40682,
                random_property_id: None,
                enchant_id: None,
                gem_ids: vec![None, None, None, None],
            }),
            trinket1: Some(CharacterItemDto {
                item_id: 37873,
                random_property_id: None,
                enchant_id: None,
                gem_ids: vec![None, None, None, None],
            }),
            trinket2: Some(CharacterItemDto {
                item_id: 37232,
                random_property_id: None,
                enchant_id: None,
                gem_ids: vec![None, None, None, None],
            }),
        },
        hero_class_id: 2,
        level: 20,
        gender: false,
        profession1: None,
        profession2: Some(3),
        talent_specialization: None,
        faction: true,
        race_id: 6,
    };
    let character_history_dto = CharacterHistoryDto {
        character_uid: 12312452,
        character_info: character_info_dto.to_owned(),
        character_name: "sdfsdgsdg".to_string(),
        guild: Some(GuildDto {
            name: "sgfdfhgdfg".to_string(),
            server_uid: 1232346,
        }),
        guild_rank: Some("BliBlaBlub".to_string()),
    };
    let character_dto = CharacterDto {
        server_uid: 12312452,
        character_history: Some(character_history_dto.to_owned()),
    };

    let num_iterations = 1000;
    let mut average_ns = Vec::new();
    for _i in 0..num_iterations {
      let start = PreciseTime::now();
      let character = armory.set_character(3, character_dto.clone()).unwrap();
      let end = PreciseTime::now();
      average_ns.push(start.to(end).num_nanoseconds().unwrap());

      // Cleanup
      let character_history = character.last_update.unwrap();
      armory.db_main.execute_wparams("DELETE FROM armory_item WHERE id=:id", params!("id" => character_history.character_info.gear.head.unwrap().id));
      armory.db_main.execute_wparams("DELETE FROM armory_item WHERE id=:id", params!("id" => character_history.character_info.gear.neck.unwrap().id));
      armory.db_main.execute_wparams("DELETE FROM armory_item WHERE id=:id", params!("id" => character_history.character_info.gear.shoulder.unwrap().id));
      armory.db_main.execute_wparams("DELETE FROM armory_item WHERE id=:id", params!("id" => character_history.character_info.gear.back.unwrap().id));
      armory.db_main.execute_wparams("DELETE FROM armory_item WHERE id=:id", params!("id" => character_history.character_info.gear.chest.unwrap().id));
      armory.db_main.execute_wparams("DELETE FROM armory_item WHERE id=:id", params!("id" => character_history.character_info.gear.wrist.unwrap().id));
      armory.db_main.execute_wparams("DELETE FROM armory_item WHERE id=:id", params!("id" => character_history.character_info.gear.main_hand.unwrap().id));
      armory.db_main.execute_wparams("DELETE FROM armory_item WHERE id=:id", params!("id" => character_history.character_info.gear.off_hand.unwrap().id));
      armory.db_main.execute_wparams("DELETE FROM armory_item WHERE id=:id", params!("id" => character_history.character_info.gear.ternary_hand.unwrap().id));
      armory.db_main.execute_wparams("DELETE FROM armory_item WHERE id=:id", params!("id" => character_history.character_info.gear.glove.unwrap().id));
      armory.db_main.execute_wparams("DELETE FROM armory_item WHERE id=:id", params!("id" => character_history.character_info.gear.belt.unwrap().id));
      armory.db_main.execute_wparams("DELETE FROM armory_item WHERE id=:id", params!("id" => character_history.character_info.gear.leg.unwrap().id));
      armory.db_main.execute_wparams("DELETE FROM armory_item WHERE id=:id", params!("id" => character_history.character_info.gear.boot.unwrap().id));
      armory.db_main.execute_wparams("DELETE FROM armory_item WHERE id=:id", params!("id" => character_history.character_info.gear.ring1.unwrap().id));
      armory.db_main.execute_wparams("DELETE FROM armory_item WHERE id=:id", params!("id" => character_history.character_info.gear.ring2.unwrap().id));
      armory.db_main.execute_wparams("DELETE FROM armory_item WHERE id=:id", params!("id" => character_history.character_info.gear.trinket1.unwrap().id));
      armory.db_main.execute_wparams("DELETE FROM armory_item WHERE id=:id", params!("id" => character_history.character_info.gear.trinket2.unwrap().id));
      armory.db_main.execute_wparams("DELETE FROM armory_gear WHERE id=:id", params!("id" => character_history.character_info.gear.id));
      armory.db_main.execute_wparams("DELETE FROM armory_character_info WHERE id=:id", params!("id" => character_history.character_info.id));
      armory.db_main.execute_wparams("DELETE FROM armory_character_history WHERE id=:id", params!("id" => character_history.id));
      armory.db_main.execute_wparams("DELETE FROM armory_character WHERE id=:id", params!("id" => character_history.character_id));
      armory.db_main.execute_wparams("DELETE FROM armory_guild WHERE id=:id", params!("id" => character_history.guild_id.unwrap()));
    }
    println!("Execution time of set_character: {} ms/iter", 0.000001 * (average_ns.iter().sum::<i64>()/num_iterations) as f64);
}