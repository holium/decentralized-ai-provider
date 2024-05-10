fyi the `crate/shared` dir is nested "pointlessly" like that so that `kit` doesn't think it's a kinode process and get mad on building it. but this way we can still share types via that lib
