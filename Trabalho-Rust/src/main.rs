use std::io::{self, BufRead, Write};
use std::thread;
use std::time::Duration;

#[derive(Clone)]
struct Codigo {
    pid: usize,
    memoria: i32,
    tempo: i32,
}

impl Codigo {
    fn new(memoria: i32, tempo: i32, pid: usize) -> Self {
        Self {
            pid,
            memoria,
            tempo,
        }
    }
}

struct Pilha {
    codigos: Vec<Codigo>,
    topo: usize,
    tot_tempo: i32,
    tot_memoria: i32,
}

impl Pilha {
    fn new(quantidade: usize) -> Self {
        Self {
            codigos: vec![Codigo::new(0, 0, 0); quantidade],
            topo: 0,
            tot_tempo: 0,
            tot_memoria: 0,
        }
    }

    fn push(&mut self, mem: i32, t: i32) {
        if self.topo >= self.codigos.len() {
            println!("Pilha cheia");
            return;
        }

        self.codigos[self.topo] = Codigo::new(mem, t, self.topo);
        self.topo += 1;

        self.tot_memoria += mem;
        self.tot_tempo += t;
    }

    fn pop(&mut self) {
        if self.topo <= 0 {
            println!("Pilha vazia");
            return;
        }

        self.topo -= 1;
        let codigoremovido = &self.codigos[self.topo];
        println!();
        println!("Processo removido: PID - {}, Memória - {}, Tempo - {}", codigoremovido.pid, codigoremovido.memoria, codigoremovido.tempo);
    }

    fn mostra_pilha(&self) {
        if self.topo == 0 {
            println!("Pilha vazia");
            return;
        }

        println!("Pilha:");
        for i in 0..self.topo {
            println!("PID's: {}", self.codigos[i].pid);
        }
        println!();
    }

    fn executa_processo(&mut self) {
        if self.topo == 0 {
            println!("Pilha vazia. Não há processos para executar.");
            return;
        }

        println!("Executando processos:");

        let mut i = self.topo;
        while i > 0 {
            i -= 1;
            let processo = &self.codigos[i];
            println!();
            println!("Iniciando execução do processo PID {} com tempo de execução {} segundos.", processo.pid, processo.tempo);

            for _ in 0..processo.tempo {
                print!("#");
                io::stdout().flush().unwrap();
                thread::sleep(Duration::from_secs(1));
            }
            println!("\nProcesso PID {} concluído.", processo.pid);
            println!();

            self.pop();
        }
    }
}

fn main() {
    let stdin = io::stdin();

    println!("Quantos processos deseja inserir?");
    let mut input = String::new();
    stdin.lock().read_line(&mut input).unwrap();
    let quantidade: usize = input.trim().parse().unwrap();

    let mut pilha = Pilha::new(quantidade);

    for i in 0..quantidade {
        println!("Insira a quantidade de memória para o processo {}: ", i + 1);
        let mut mem_input = String::new();
        stdin.lock().read_line(&mut mem_input).unwrap();
        let mem: i32 = mem_input.trim().parse().unwrap();

        println!("Insira o tempo de execução para o processo {}: ", i + 1);
        let mut t_input = String::new();
        stdin.lock().read_line(&mut t_input).unwrap();
        let t: i32 = t_input.trim().parse().unwrap();

        if t < 30 || t > 90 {
            println!("Insira um intervalo de tempo válido entre 30 e 90 segundos");
            return;
        }

        pilha.push(mem, t);
    }

    pilha.mostra_pilha();
    pilha.executa_processo();
    pilha.mostra_pilha();

    println!();

    println!("Total tempo de execução: {} segundos", pilha.tot_tempo);
    println!("Total memória utilizada: {} MB", pilha.tot_memoria);
}