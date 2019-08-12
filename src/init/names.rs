use crate::{
	dir, init::{PathDialog, ASK_FOR_PATH, PROJECT_NAME_TEMPLATE}, interpolation::Interpolation
};
use evscode::R;
use std::{
	fmt, path::{Path, PathBuf}, str::FromStr
};
use unijudge::TaskDetails;

/// Default contest directory name. This key uses special syntax to allow using dynamic content, like contest ids. See "Icie Init Project name template" for details.
#[evscode::config]
static CONTEST: evscode::Config<Interpolation<ContestVariable>> = "{site.short case.camel}-{contest.id case.camel}".parse().unwrap();

/// Default task directory name, when created as a part of a contest. This key uses special syntax to allow using dynamic content, like task titles. See "Icie Init Project name template" for details.
#[evscode::config]
static CONTEST_TASK: evscode::Config<Interpolation<ContestTaskVariable>> = "{task.symbol case.upper}-{task.name case.kebab}".parse().unwrap();

pub fn design_task_name(root: &Path, meta: Option<&TaskDetails>) -> R<PathBuf> {
	let variables = Mapping {
		task_id: meta.as_ref().map(|meta| meta.id.clone()),
		task_title: meta.as_ref().map(|meta| meta.title.clone()),
		contest_id: meta.as_ref().map(|meta| meta.contest_id.clone()),
		site_short: meta.as_ref().map(|meta| meta.site_short.clone()),
	};
	let (codename, all_good) = PROJECT_NAME_TEMPLATE.get().interpolate(&variables);
	let config_strategy = ASK_FOR_PATH.get();
	let strategy = match (&*config_strategy, all_good) {
		(_, false) => &PathDialog::InputBox,
		(s, true) => s,
	};
	//	strategy.query(&*dir::PROJECT_DIRECTORY.get(), &codename)
	strategy.query(root, &codename)
}

pub fn design_contest_name(contest_id: &str, site_short: &'static str) -> R<PathBuf> {
	let variables = Mapping { task_id: None, task_title: None, contest_id: Some(contest_id.to_owned()), site_short: Some(site_short.to_owned()) };
	let (codename, all_good) = CONTEST.get().interpolate(&variables);
	let config_strategy = ASK_FOR_PATH.get();
	let strategy = match (&*config_strategy, all_good) {
		(_, false) => &PathDialog::InputBox,
		(s, true) => s,
	};
	strategy.query(&*dir::PROJECT_DIRECTORY.get(), &codename)
}

pub enum Variable {
	RandomCute,
	RandomAnimal,
	TaskId,
	TaskTitle,
	ContestId,
	SiteShort,
}

pub struct Mapping {
	pub task_id: Option<String>,
	pub task_title: Option<String>,
	pub contest_id: Option<String>,
	pub site_short: Option<String>,
}

macro_rules! constrain_variable {
	($name:ident, $($matching:ident)|*) => {
		pub struct $name(Variable);
		impl crate::interpolation::VariableSet for $name {
			type Map = Mapping;

			fn expand(&self, map: &Self::Map) -> Option<String> {
				crate::interpolation::VariableSet::expand(&self.0, map)
			}
		}
		impl std::str::FromStr for $name {
			type Err = String;

			fn from_str(s: &str) -> Result<Self, Self::Err> {
				let v = $name(Variable::from_str(s)?);
				match v.0 {
					$(Variable::$matching => Ok($name(Variable::$matching)),)*
					#[allow(unreachable_patterns)]
					_ => Err(format!("variable {} not supported in {} context", v, stringify!($name))),
				}
			}
		}
		impl fmt::Display for $name {
			fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
				self.0.fmt(f)
			}
		}
	};
}

constrain_variable!(TaskVariable, RandomCute | RandomAnimal | TaskId | TaskTitle | ContestId | SiteShort);
constrain_variable!(ContestVariable, RandomCute | RandomAnimal | ContestId | SiteShort);
constrain_variable!(ContestTaskVariable, RandomCute | RandomAnimal | TaskId | TaskTitle | ContestId | SiteShort);

impl crate::interpolation::VariableSet for Variable {
	type Map = Mapping;

	fn expand(&self, map: &Self::Map) -> Option<String> {
		match self {
			Variable::RandomCute => Some(crate::dir::random_adjective().to_owned()),
			Variable::RandomAnimal => Some(crate::dir::random_animal().to_owned()),
			Variable::TaskId => map.task_id.clone(),
			Variable::TaskTitle => map.task_title.clone(),
			Variable::ContestId => map.contest_id.clone(),
			Variable::SiteShort => map.site_short.clone(),
		}
	}
}

impl FromStr for Variable {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"random.cute" => Ok(Variable::RandomCute),
			"random.animal" => Ok(Variable::RandomAnimal),
			"task.symbol" => Ok(Variable::TaskId),
			"task.name" => Ok(Variable::TaskTitle),
			"contest.id" => Ok(Variable::ContestId),
			"site.short" => Ok(Variable::SiteShort),
			_ => Err(format!("unrecognized variable name {:?}", s)),
		}
	}
}

impl fmt::Display for Variable {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_str(match self {
			Variable::RandomCute => "random.cute",
			Variable::RandomAnimal => "random.animal",
			Variable::TaskId => "task.symbol",
			Variable::TaskTitle => "task.name",
			Variable::ContestId => "contest.id",
			Variable::SiteShort => "site.short",
		})
	}
}