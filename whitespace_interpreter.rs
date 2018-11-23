use std::env;
use std::fs;
use std::collections::HashMap;
use std::io::stdin;
use std::io::Read;

#[derive(Debug, PartialEq)]
enum WSC { S, T, L }

// ASCIIコード列を S, T, L の列に変換
fn convert(v: Vec<u8>) -> Vec<WSC> {
  let mut new_vec = Vec::new();
  for i in v {
    match i {
      32 => new_vec.push(WSC::S),
      9  => new_vec.push(WSC::T),
      10 => new_vec.push(WSC::L),
      _  => {} // 他の文字は無視
    }
  }
  new_vec
}

// 数値変換
fn calc_num(mut v: Vec<WSC>) -> i32 {
  let mut ans = 0;

  match &v[0] {
    WSC::S => {
      v.remove(0);
      for i in v {
        match i {
          WSC::S => { ans = ans * 2; }
          WSC::T => { ans = ans * 2 + 1; }
          WSC::L => { break; }
        }
      }
    }
    WSC::T => {
      v.remove(0);
      for i in v {
        match i {
          WSC::S => { ans = ans * 2; }
          WSC::T => { ans = ans * 2 + 1; }
          WSC::L => { break; }
        }
      }
      ans *= -1;
    }
    WSC::L => { panic!("数値が不正です."); }
  }

  ans
}
 
fn main() {
  let filename = match env::args().nth(1) {
    Some(x) => {x}
    None => { panic!("引数を指定してください."); }
  };

  let content = match fs::read_to_string(filename) {
    Ok(file) => {file}
    Err(why) => { panic!(why.to_string()) }
  };

  let byte_vector = String::into_bytes(content);
  let wsc_v = convert(byte_vector);
  // println!(" For debug: wsc_v = {:?}", wsc_v);

  // 命令の位置, スタック, ヒープ, ラベル, コールスタック
  let mut i = 0;
  let mut stack = Vec::new();
  let mut heap = HashMap::new();
  let mut callstack = Vec::new();
  let mut caller: usize;

  // 標準入力関係
  let stdin = stdin();
  let mut handle = stdin.lock();
  let mut buffer = [0; 1];

  loop {
    // println!(" For debug: i = {}", i);
    // println!(" For debug: stack = {:?}", stack);
    // println!(" For debug: callstack = {:?}", callstack);
    // println!(" For debug: heap = {:?}", heap);

    if i >= wsc_v.len() {
      return;
    }

    match &wsc_v[i] {
      // スタック操作
      WSC::S => {
        i += 1;

        match &wsc_v[i] {
          // スタックに与えられた数値を積む
          WSC::S => {
            // println!(" For debug: [SS]");
            let mut num_v = Vec::new();
            loop {
              i += 1;
              match &wsc_v[i] {
                WSC::S => { num_v.push(WSC::S); }
                WSC::T => { num_v.push(WSC::T); }
                WSC::L => { num_v.push(WSC::L); break; }
              }
            }

            stack.push(calc_num(num_v));
          }

          WSC::L => {
            i += 1;
            match &wsc_v[i] {
              // スタックの先頭と同じ値をさらにスタックに積む
              WSC::S => {
                // println!(" For debug: [SLS]");
                match stack.pop() {
                  Some(x) => {
                    stack.push(x);
                    stack.push(x);
                  }
                  None => {} // スタックの先頭に何も無いので何もしない
                }
              }

              // スタックの先頭要素と次の要素を入れ替える
              WSC::T => {
                // println!(" For debug: [SLT]");
                match stack.pop() {
                  Some(a) => {
                    match stack.pop() {
                      Some(b) => {
                        stack.push(a);
                        stack.push(b);
                      }
                      None => {
                        stack.push(a); // スタックの次の要素が無いのでそのまま戻す
                      }
                    }
                  }
                  None => {} // スタックの先頭に何も無いので何もしない
                }
              }

              // スタックの先頭要素を取り除く
              WSC::L => { 
                // println!(" For debug: [SLL]");
                stack.pop(); 
              }
            }
          }

          WSC::T => { panic!( "入力[ST]は不正です." ); }
        }
      }

      WSC::T => {
        i += 1;

        match &wsc_v[i] {
          // 算術演算
          WSC::S => {
            i += 1;

            match &wsc_v[i] {
              WSC::S => {
                i += 1;

                match &wsc_v[i] {
                  // 加算
                  WSC::S => {
                    // println!(" For debug: [TSSS]");
                    match stack.pop() {
                      Some(a) => {
                        match stack.pop() {
                          Some(b) => {
                            stack.push(b + a);
                          }
                          None => {
                            stack.push(a); // スタックの次の要素が無いのでそのまま戻す
                          }
                        }
                      }
                      None => {} // スタックの先頭に何も無いので何もしない
                    }
                  }

                  // 減算
                  WSC::T => {
                    // println!(" For debug: [TSST]");
                    match stack.pop() {
                      Some(a) => {
                        match stack.pop() {
                          Some(b) => {
                            stack.push(b - a);
                          }
                          None => {
                            stack.push(a); // スタックの次の要素が無いのでそのまま戻す
                          }
                        }
                      }
                      None => {} // スタックの先頭に何も無いので何もしない
                    }
                  }

                  // 乗算
                  WSC::L => {
                    // println!(" For debug: [TSSL]");
                    match stack.pop() {
                      Some(a) => {
                        match stack.pop() {
                          Some(b) => {
                            stack.push(b * a);
                          }
                          None => {
                            stack.push(a); // スタックの次の要素が無いのでそのまま戻す
                          }
                        }
                      }
                      None => {} // スタックの先頭に何も無いので何もしない
                    }
                  }                  
                }
              }  

              WSC::T => {
                i += 1;

                match &wsc_v[i] {
                  // 除算
                  WSC::S => {
                    // println!(" For debug: [TSTS]");
                    match stack.pop() {
                      Some(a) => {
                        match stack.pop() {
                          Some(b) => {
                            stack.push(b / a);
                          }
                          None => {
                            stack.push(a); // スタックの次の要素が無いのでそのまま戻す
                          }
                        }
                      }
                      None => {} // スタックの先頭に何も無いので何もしない
                    }
                  }

                  // 剰余算
                  WSC::T => {
                    // println!(" For debug: [TSTT]");
                    match stack.pop() {
                      Some(a) => {
                        match stack.pop() {
                          Some(b) => {
                            stack.push(b % a);
                          }
                          None => {
                            stack.push(a); // スタックの次の要素が無いのでそのまま戻す
                          }
                        }
                      }
                      None => {} // スタックの先頭に何も無いので何もしない
                    }
                  }

                  WSC::L => { panic!( "入力[TSTL]は不正です." ); }           
                }
              }

              WSC::L => { panic!( "入力[TSL]は不正です." ); }
            }
          }

          // ヒープ操作
          WSC::T => {
            i += 1;

            match &wsc_v[i] {
              // 格納
              WSC::S => {
                // println!(" For debug: [TTS]");
                match stack.pop() {
                  Some(v) => {
                    match stack.pop() {
                      Some(k) => {
                        heap.insert(k, v);
                      }
                      None => {
                        stack.push(v); // スタックの次の要素が無いのでそのまま戻す
                      }
                    }
                  }
                  None => {} // スタックの先頭に何も無いので何もしない
                }
              }

              // 取り出し
              WSC::T => {
                // println!(" For debug: [TTT]");
                match stack.pop() {
                  Some(k) => {
                    stack.push(heap[&k]);
                  }
                  None => {} // スタックの先頭に何も無いので何もしない
                }
              }

              WSC::L => { panic!( "入力[TTL]は不正です." ); }
            }
          }

          // 入出力
          WSC::L => {
            i += 1;

            match &wsc_v[i] {
              WSC::S => {
                i += 1;

                match &wsc_v[i] {
                  // スタックの先頭要素を文字として出力
                  WSC::S => {
                    // println!(" For debug: [TLSS]");
                    match stack.pop() {
                      Some(x) => {
                        print!("{}", x as u8 as char);
                      }
                      None => {} // スタックの先頭に何も無いので何もしない
                    }
                  }

                  // スタックの先頭要素を整数として出力
                  WSC::T => {
                    // println!(" For debug: [TLST]");
                    match stack.pop() {
                      Some(x) => {
                        print!("{}", x);
                      }
                      None => {} // スタックの先頭に何も無いので何もしない
                    }
                  }

                  WSC::L => { panic!( "入力[TLSL]は不正です." ); }
                }
              }

              WSC::T => {
                i += 1;

                match &wsc_v[i] {
                  // スタックの先頭要素が指すアドレスへ文字として入力を受け取る
                  WSC::S => {
                    // println!(" For debug: [TLTS]");
                    loop {
                      match handle.read(&mut buffer) {
                        Ok(n) => {
                          if n == 1 {
                            let v = buffer[0];

                            if v != 10 { // 改行文字は除く
                              // println!(" For debug: 文字コード {} を読み取りました", v); 

                              match stack.pop() {
                                Some(k) => {
                                  heap.insert(k, v as i32);
                                }
                                None => {} // スタックの先頭に何も無いので何もしない
                              }

                              break;
                            }
                          }
                        }

                        Err(why) => { panic!(why.to_string()) }
                      }
                    }
                  }

                  // スタックの先頭要素が指すアドレスへ整数として入力を受け取る
                  WSC::T => {
                    // println!(" For debug: [TLTT]");
                    let mut keta = Vec::new();
                    let mut b = false;
                    let mut is_neg = false;

                    loop {
                      match handle.read(&mut buffer) {
                        Ok(n) => {
                          if n == 1 {
                            let v = buffer[0];

                            // 先頭がマイナスなら負に
                            if v == 45 && b == false {
                              is_neg = true;
                              continue;
                            }

                            // 空白文字で終了
                            if v == 32 && b {
                              break;
                            }

                            // 改行文字で終了
                            if v == 10 && b {
                              break;
                            }

                            // まだ読んでいない状態で空白文字、改行文字なら読み飛ばす
                            if v == 32 || v == 10 {
                              continue;
                            }

                            if v >= 48 && v <= 57 {
                              keta.push((v-48) as i32);
                              b = true;
                            } else {
                              panic!( "数値以外を読み取ろうとしています." );
                            }
                          }
                        }

                        Err(why) => { panic!(why.to_string()) }
                      }
                    }

                    let mut num: i32 = keta.iter().fold(0, |acc, x| acc * 10 + x);
                    if is_neg { num *= -1; }

                    match stack.pop() {
                      Some(k) => {
                        heap.insert(k, num);
                      }
                      None => {} // スタックの先頭に何も無いので何もしない
                    }

                    // println!(" For debug: 数値 {} を読み取りました", num); 
                  }

                  WSC::L => { panic!( "入力[TLTL]は不正です." ); }
                }
              }

              WSC::L => { panic!( "入力[TLL]は不正です." ); }
            }
          }
        }
      }

      // フロー制御
      WSC::L => {
        i += 1;

        match &wsc_v[i] {
          WSC::S => {
            i += 1;

            match &wsc_v[i] {
              // 与えられたラベルを命令の置かれた場所に付ける
              WSC::S => {
                // println!(" For debug: [LSS]");
                let mut label_v = Vec::new();
                loop {
                  i += 1;
                  match &wsc_v[i] {
                    WSC::S => { label_v.push(WSC::S); }
                    WSC::T => { label_v.push(WSC::T); }
                    WSC::L => { label_v.push(WSC::L); break; }
                  }
                }
              }

              // 関数呼び出し
              WSC::T => {
                // println!(" For debug: [LST]");
                let mut func_v = Vec::new();
                loop {
                  i += 1;
                  match &wsc_v[i] {
                    WSC::S => { func_v.push(WSC::S); }
                    WSC::T => { func_v.push(WSC::T); }
                    WSC::L => {
                      func_v.push(WSC::L);
                      caller = i;
                      callstack.push(caller);
                      break; 
                    }
                  }
                }   
                
                let len = func_v.len();
                let mut tmp_v = Vec::new();

                i = 0;
                
                loop {
                  match &wsc_v[i] {
                    WSC::S => { tmp_v.push(WSC::S); }
                    WSC::T => { tmp_v.push(WSC::T); }
                    WSC::L => { tmp_v.push(WSC::L); }
                  }

                  if tmp_v.len() > len { tmp_v.remove(0); }

                  if tmp_v == func_v && caller != i { 
                    break; 
                  }

                  i += 1;
                }
              }

              // ラベルが示す先に無条件ジャンプ
              WSC::L => {
                // println!(" For debug: [LSL]");
                let mut label_v = Vec::new();
                loop {
                  i += 1;
                  match &wsc_v[i] {
                    WSC::S => { label_v.push(WSC::S); }
                    WSC::T => { label_v.push(WSC::T); }
                    WSC::L => { label_v.push(WSC::L); break; }
                  }
                }

                let len = label_v.len() + 3;
                label_v.insert(0, WSC::S);
                label_v.insert(0, WSC::S);
                label_v.insert(0, WSC::L);
                let mut tmp_v = Vec::new();

                i = 0;
                
                loop {
                  match &wsc_v[i] {
                    WSC::S => { tmp_v.push(WSC::S); }
                    WSC::T => { tmp_v.push(WSC::T); }
                    WSC::L => { tmp_v.push(WSC::L); }
                  }

                  if tmp_v.len() > len { tmp_v.remove(0); }

                  if tmp_v == label_v { 
                    break; 
                  }

                  i += 1;
                }
              }
            }
          }

          WSC::T => {
            i += 1;

            match &wsc_v[i] {
              // ラベルが示す先にスタックの先頭要素が 0 ならジャンプ
              WSC::S => {
                // println!(" For debug: [LTS]");
                let mut label_v = Vec::new();
                loop {
                  i += 1;
                  match &wsc_v[i] {
                    WSC::S => { label_v.push(WSC::S); }
                    WSC::T => { label_v.push(WSC::T); }
                    WSC::L => { label_v.push(WSC::L); break; }
                  }
                }

                match stack.pop() {
                  Some(x) => {
                    if x == 0 {
                      let len = label_v.len() + 3;
                      label_v.insert(0, WSC::S);
                      label_v.insert(0, WSC::S);
                      label_v.insert(0, WSC::L);
                      let mut tmp_v = Vec::new();

                      i = 0;
                      
                      loop {
                        match &wsc_v[i] {
                          WSC::S => { tmp_v.push(WSC::S); }
                          WSC::T => { tmp_v.push(WSC::T); }
                          WSC::L => { tmp_v.push(WSC::L); }
                        }

                        if tmp_v.len() > len { tmp_v.remove(0); }

                        if tmp_v == label_v { 
                          break; 
                        }

                        i += 1;
                      }
                    }
                  }
                  None => {} // スタックの先頭に何も無いので何もしない
                }
              }

              // ラベルが示す先にスタックの先頭要素が負ならジャンプ
              WSC::T => {
                // println!(" For debug: [LTT]");
                let mut label_v = Vec::new();
                loop {
                  i += 1;
                  match &wsc_v[i] {
                    WSC::S => { label_v.push(WSC::S); }
                    WSC::T => { label_v.push(WSC::T); }
                    WSC::L => { label_v.push(WSC::L); break; }
                  }
                }

                match stack.pop() {
                  Some(x) => {
                    if x < 0 {
                      let len = label_v.len() + 3;
                      label_v.insert(0, WSC::S);
                      label_v.insert(0, WSC::S);
                      label_v.insert(0, WSC::L);
                      let mut tmp_v = Vec::new();

                      i = 0;
                      
                      loop {
                        match &wsc_v[i] {
                          WSC::S => { tmp_v.push(WSC::S); }
                          WSC::T => { tmp_v.push(WSC::T); }
                          WSC::L => { tmp_v.push(WSC::L); }
                        }

                        if tmp_v.len() > len { tmp_v.remove(0); }

                        if tmp_v == label_v { 
                          break; 
                        }

                        i += 1;
                      }
                    }
                  }
                  None => {} // スタックの先頭に何も無いので何もしない
                }
              }

              // 関数を終了して caller の元へ帰る
              WSC::L => {
                // println!(" For debug: [LTL]");
                match callstack.pop() {
                  Some(x) => {
                    i = x;
                  }
                  None => { panic!( "コールスタックに何もありません." ); }
                }
              }
            }
          }

          WSC::L => {
            i += 1;

            match &wsc_v[i] {
              WSC::S => { panic!( "入力[LLS]は不正です." ); }
              WSC::T => { panic!( "入力[LLT]は不正です." ); }
              WSC::L => { 
                // println!(" For debug: [LLL]");
                return; 
              }
            }
          }
        }
      }

    }

    i += 1;
    // println!(" For debug: ");
  }
}
