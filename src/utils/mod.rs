use std::{future::Future, vec};

pub mod thread_pool;


pub struct Worker {
      id: usize,
      thread: Option<tokio::task::JoinHandle<()>>,
}

impl Worker
{
      pub fn new(id: usize, job: impl Future<Output = ()> + Send + 'static) -> Self
      {
            Self {
                  id,
                  thread: Some(tokio::spawn(job)),
            }
      }
}

impl Drop for Worker {
      fn drop(&mut self) {
            if let Some(thread) = self.thread.take() {
                  thread.abort();
            }
      }
}

#[macro_export]
macro_rules! filter_str_by {
      ($str:expr,$($expr:expr),*) => {
            {
                  let mut temp_vec = Vec::<String>::new();
                  $(
                        let result = $crate::utils::filter_str_by_expr($str,$expr);
                        if result.is_ok() {
                              temp_vec.append(&mut result.unwrap());
                        }
                  )*
                  temp_vec
            }
      };
}

#[macro_export]
macro_rules! if_nothing_then {
      ($a:expr,$b:expr) => {
            {
                  if $a.len() == 0 {
                        $b
                  } else {
                        $a
                  }
            }
      };
      ($a:expr,($($b:expr),*),$c:expr) => {
            {
                  if $a.len() == 0 {
                        return $c;
                  } else if {
                        $(
                              if $b.len() == 0 {
                                    return $c;
                              }
                        )*
                  } else {
                        return $a;
                  }
            }
      };
}

pub fn filter_str_by_expr(str: &str, pattern: &str) -> Result<Vec<String>,()> {
      let splited_pattern: Vec<&str> = pattern.split("{}").collect();
      if splited_pattern.len() != 2 {
            return Err(());
      }

      let left_indices = get_matching_indices(str, &splited_pattern, 0);
      let right_indices = get_matching_indices(str, &splited_pattern, 1);

      let mut result: Vec<String> = Vec::new();

      for l_index in left_indices.iter() {
            for r_index in right_indices.iter() {
                  if *l_index <= *r_index {
                        result.push(String::from(&str[*l_index..*r_index]));
                        break;
                  }
            }
      }

      Ok(result)
}


pub fn find_many(str: &str, find: &str, offset: usize) -> Vec<usize> {
      let mut result: Vec<usize> = Vec::new();
      let mut m_str = String::from(str);
      let mut last_index = 0usize;
      'a: loop {
            if let Some(index) = m_str.find(find) {
                  result.push(index+offset+last_index);
                  last_index += index+find.len();
                  m_str = String::from(&m_str.as_str()[index+find.len()..]);
                  continue 'a;
            }
            break 'a;
      }
      result
}

fn get_matching_indices(str: &str, pattern: &Vec<&str>, pattern_side: usize) -> Vec<usize> {
      if pattern_side != 0 && pattern_side != 1 {
            panic!("Invalid pattern side");
      }
      let side = pattern.get(pattern_side).unwrap();
      if side.len()==0 {
            if pattern_side==0 {return vec![0];}
            else {return vec![str.len()]}
      } else {
            return find_many(str, side, if pattern_side==0 {side.len()} else {0});
      };
}

pub fn if_nothing_then<T>(vector: Vec<T>, then: Vec<T>) -> Vec<T> {
      if vector.len() == 0 {
            return then;
      }
      vector
}

pub fn str_to_vec_usize(str: &str) -> Vec<usize> {
      let mut result: Vec<usize> = Vec::new();
      for a in str.as_bytes() {
            result.push(*a as usize);
      }
      result
}
