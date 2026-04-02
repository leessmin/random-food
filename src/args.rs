use clap::{Arg, ArgAction, ArgMatches, Command};
use rand::seq::SliceRandom;

use crate::{
    FOOD_MENU_CATEGORY,
    crawler::{self, Food},
};

const ABOUT: &'static str = r#"今天吃什么？
一个随机食物生成器。
使用方法：
random-food (随机生成一个食物)
random-food list (查看菜单)
random-food -o 素菜 荤菜 -c 2 (查看食物种类)"#;
const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const HELP_TEMPLATE: &'static str = r#"
{about}

用法: {bin} [命令] [选项]

命令:
{subcommands}

选项:
{options}
"#;

fn cli() -> ArgMatches {
    Command::new("random-food")
        .about(ABOUT)
        .version(VERSION)
        .subcommand_required(false)
        .disable_help_flag(true)
        .disable_version_flag(true)
        .disable_help_subcommand(true)
        // .mut_arg("help", |a| a.help("显示帮助信息"))
        // .mut_arg("version", |a| a.help("显示版本信息"))
        .arg(
            Arg::new("help")
                .short('h')
                .long("help")
                .help("显示帮助信息")
                .action(ArgAction::Help),
        )
        .arg(
            Arg::new("version")
                .short('V')
                .long("version")
                .help("显示版本信息")
                .action(ArgAction::Version),
        )
        .arg(
            Arg::new("option")
                .short('o')
                .long("option")
                .num_args(0..)
                .help("需要添加的菜单种类列表，默认全部"),
        )
        .arg(
            Arg::new("count")
                .short('c')
                .long("count")
                .help("输出的食物数量, 默认1")
                .default_value("1")
                .value_parser(clap::value_parser!(usize)),
        )
        .subcommand(Command::new("list").short_flag('l').about("食物种类列表"))
        // .subcommand(Command::new("count").short_flag('c').about("食物数量"))
        .help_template(HELP_TEMPLATE)
        .get_matches()
}

fn list_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    let menus = crawler::get_food_menu()?;

    println!("菜单种类如下: ");
    menus
        .keys()
        .for_each(|menu| println!("{}({})", menu, menus.get(menu).unwrap_or(&vec![]).len()));
    println!("=====>>>>><<<<<=====");
    println!(
        "菜单数量: {} || 食物总数: {}",
        menus.keys().collect::<Vec<&String>>().len(),
        menus.values().flatten().count()
    );

    Ok(())
}

fn global_command(matches: ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let option = matches
        .get_many::<String>("option")
        .map(|vals| vals.map(|s| s.as_str()).collect())
        .unwrap_or(FOOD_MENU_CATEGORY.to_vec());
    let count = matches.get_one::<usize>("count").unwrap_or(&1);

    let menus = crawler::get_food_menu()?;

    let mut foods: Vec<&Food> = menus
        .iter()
        .filter(|(key, _)| option.contains(&key.as_str()))
        .flat_map(|(_, values)| values.iter())
        .collect();

    foods.shuffle(&mut rand::rng());
    // https://raw.githubusercontent.com/Anduin2017/HowToCook/refs/heads/master/
    // https://github.com/Anduin2017/HowToCook/blob/master/
    foods.iter().take(*count).for_each(|food| {
        println!(
            "{} -> https://raw.githubusercontent.com/Anduin2017/HowToCook/refs/heads/master/{}",
            food.name, food.courses
        )
    });

    Ok(())
}

pub fn exec_args() -> Result<(), Box<dyn std::error::Error>> {
    // Args::parse()
    let matches = cli();

    match matches.subcommand() {
        Some(("list", _)) => list_subcommand(),
        _ => global_command(matches),
    }
}
