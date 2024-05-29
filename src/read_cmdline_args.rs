// read_cmdline_args: Gets a list of command-line parameters (2022-04-06 bar8tl)
use std::env;

#[derive(Debug, Clone, Default)]
pub struct ParameTp {
  pub optn: String,
  pub prm1: String,
  pub prm2: String,
  pub prm3: String
}

#[derive(Debug, Clone, Default)]
pub struct ParamsTp {
  pub cmdpr: Vec<ParameTp>,
  pub messg: String
}

pub fn read_cmdline_args() -> ParamsTp {
  let mut prm: ParamsTp = Default::default();
  let args: Vec<String> = env::args().collect();
  if args.len() == 1 {
    prm.messg = String::from("Run option missing");
    return prm;
  }
  let mut argn: i32 = 0;
  for curarg in args {
    if argn > 0 {
      if curarg[0..1] == "-".to_string() || curarg[0..1] == "/".to_string() {
        let mut optn: String = curarg[1..].trim().to_lowercase();
        let mut prm1: String = "".to_string();
        let mut prm2: String = "".to_string();
        let mut prm3: String = "".to_string();
        if optn != "".to_string() {
          let idx = optn.find(":");
          if idx != None {
            let i = idx.unwrap();
            prm1 = optn[i + 1..].trim().to_string();
            optn = optn[..i].trim().to_string();
            let idx = prm1.find(":");
            if idx != None {
              let j = idx.unwrap();
              prm2 = prm1[j + 1..].trim().to_string();
              prm1 = prm1[..j].trim().to_string();
              let idx = prm2.find(":");
              if idx != None {
                let k = idx.unwrap();
                prm3 = prm2[k + 1..].trim().to_string();
                prm2 = prm2[..k].trim().to_string();
              }
            }
          }
          prm.cmdpr.push(ParameTp { optn, prm1, prm2, prm3 });
        }
      } else {
        prm.messg = String::from("Run option missing");
        return prm;
      }
    }
    argn += 1;
  }
  return prm;
}
