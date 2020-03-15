use log::debug;
use pewcraft_common::game_definition::class::{Class, ClassId};
use pewcraft_common::game_definition::effect::Effect;
use pewcraft_common::game_definition::map::GameMap;
use pewcraft_common::game_definition::skill::{Skill, SkillId};
use pewcraft_common::game_definition::GameDefinition;
use pewcraft_common::id::MapBuilder;
use serde_json::from_reader;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

const SKILLS_FILE: &str = "skills.json";
const BUFFS_FILE: &str = "effects.json";
const CLASS_FILE: &str = "class.json";

const CLASSES_DIR: &str = "classes";
const MAPS_DIR: &str = "maps";

fn load_map<P: AsRef<Path>>(map_file: P) -> GameMap {
    debug!("load_map from: {:?}", map_file.as_ref());
    let file = fs::File::open(map_file).unwrap();
    from_reader(file).unwrap()
}

fn load_maps<P: AsRef<Path>>(maps_dir: P) -> MapBuilder<GameMap> {
    debug!("load_maps from: {:?}", maps_dir.as_ref());
    let mut maps = MapBuilder::new();
    for entry in fs::read_dir(maps_dir).unwrap() {
        let entry = entry.unwrap();
        let map_file = entry.path();

        let mut map = load_map(map_file);
        map.check_validity().unwrap();
        maps.add(map);
    }
    maps
}

fn load_skills<P: AsRef<Path>>(skills_file: P) -> Vec<Skill> {
    debug!("load_skills from: {:?}", skills_file.as_ref());
    let file = fs::File::open(skills_file).unwrap();
    from_reader(file).unwrap()
}

fn load_effects<P: AsRef<Path>>(effects_file: P) -> Vec<Effect> {
    debug!("load_effects from: {:?}", effects_file.as_ref());
    let file = fs::File::open(effects_file).unwrap();
    from_reader(file).unwrap()
}

fn load_class<P: AsRef<Path>>(class_file: P) -> Class {
    debug!("load_class from: {:?}", class_file.as_ref());
    let file = fs::File::open(class_file).unwrap();
    from_reader(file).unwrap()
}

pub fn load<P: AsRef<Path>>(dir: P) -> GameDefinition {
    debug!("load game_definition from: {:?}", dir.as_ref());
    let dir = dir.as_ref();
    let mut class_to_skills = HashMap::<ClassId, Vec<SkillId>>::new();
    let mut skill_to_classes = HashMap::new();
    let mut class_builder = MapBuilder::new();
    let mut skill_builder = MapBuilder::new();
    let mut effect_builder = MapBuilder::new();

    for entry in fs::read_dir(dir.join(CLASSES_DIR)).unwrap() {
        let entry = entry.unwrap();
        let class_dir = entry.path();

        let skills = load_skills(class_dir.join(SKILLS_FILE));
        let class = load_class(class_dir.join(CLASS_FILE));
        let effects = load_effects(class_dir.join(BUFFS_FILE));

        for effect in effects {
            effect_builder.add(effect);
        }

        let class_id = class_builder.add(class);
        let usable_skills = skills
            .into_iter()
            .map(|s| {
                let skill_id = skill_builder.add(s);
                skill_to_classes.insert(skill_id, vec![class_id]);
                skill_id
            })
            .collect();
        class_to_skills.insert(class_id, usable_skills);
    }

    ///////// Add all the skills common to all classes
    // Must be done after loading all the classes
    let common_skills = load_skills(dir.join(SKILLS_FILE));
    let all_classes_id: Vec<_> = class_to_skills
        .iter()
        .map(|(class_id, _)| *class_id)
        .collect();
    for skill in common_skills {
        let skill_id = skill_builder.add(skill);

        for allowed_skills in class_to_skills.values_mut() {
            allowed_skills.push(skill_id);
        }

        skill_to_classes.insert(skill_id, all_classes_id.clone());
    }

    let common_effects = load_effects(dir.join(BUFFS_FILE));
    for effect in common_effects {
        effect_builder.add(effect);
    }

    let maps = load_maps(dir.join(MAPS_DIR)).build();

    GameDefinition {
        classes: class_builder.build(),
        skills: skill_builder.build(),
        effects: effect_builder.build(),
        class_to_skills,
        skill_to_classes,
        maps,
    }
}
