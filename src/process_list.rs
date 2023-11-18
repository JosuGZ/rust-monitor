use std::collections::HashMap;

use super::proc::Proc;

pub struct ProcessList {
  lists: [HashMap<i32, Proc>; 2],
  last_list: usize
}

impl ProcessList {

  pub fn new() -> ProcessList {
    ProcessList {
      lists: Default::default(),
      last_list: 1
    }
  }

  pub fn on_list(&mut self, list: &mut Vec<Proc>) {
    let current_list = if self.last_list == 1 { 0 } else { 1 };

    for process in &mut list.iter_mut() {
      let pid = process.pid;

      self.lists[current_list].insert(pid, process.clone());

      if let Some(last_instance) = self.lists[self.last_list].remove(&pid) {
        *process -= last_instance;
      } else {
        process.new = true;
      }
    }

    // Todo: la CPU es incorrecta para los borrados al no haber resta
    for deleted in &mut self.lists[self.last_list] {
      deleted.1.deleted = true;
      let proc = deleted.1;
      list.push(proc.clone());
    }

    self.lists[self.last_list].clear();

    self.last_list = current_list;
  }
}
