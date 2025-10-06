use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<BattleState>()
        .add_message::<BattleType>()
        .add_systems(Startup, setup)
        .add_systems(
            OnEnter(BattleState::Initialization),
            handle_battle_initialization,
        )
        .add_systems(OnEnter(BattleState::Start), handle_battle_start)
        .add_systems(
            OnEnter(BattleState::RoundStartPhase),
            handle_battle_round_start_phase,
        )
        .add_systems(
            OnEnter(BattleState::PlayerInputPhase),
            handle_battle_player_input_phase,
        )
        .add_systems(
            OnEnter(BattleState::ActionOrderingPhase),
            handle_battle_action_ordering_phase,
        )
        .add_systems(
            OnEnter(BattleState::ActionExecutionPhase),
            handle_battle_action_execution_phase,
        )
        .add_systems(
            OnEnter(BattleState::FaintResolutionPhase),
            handle_battle_faint_resolution_phase,
        )
        .add_systems(
            OnEnter(BattleState::RoundEndPhase),
            handle_battle_round_end_phase,
        )
        .add_systems(OnEnter(BattleState::End), handle_battle_end)
        .add_systems(OnExit(BattleState::End), handle_battle_end_exit)
        .run();
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum BattleState {
    #[default]
    NoBattle,
    Initialization,
    Start,
    RoundStartPhase,
    PlayerInputPhase,
    ActionOrderingPhase,
    ActionExecutionPhase,
    FaintResolutionPhase,
    RoundEndPhase,
    End,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Message)]
enum BattleType {
    PvP,
    PvE,
}

fn setup(mut battle_state: ResMut<NextState<BattleState>>) {
    battle_state.set(BattleState::Initialization);
    println!("启动战斗");
}

fn handle_battle_initialization(
    mut next_state: ResMut<NextState<BattleState>>,
    mut battle_type: MessageWriter<BattleType>,
) {
    println!("战斗初始化");
    // 设置战斗类型
    battle_type.write(BattleType::PvE);
    next_state.set(BattleState::Start);
}

fn handle_battle_start(mut next_state: ResMut<NextState<BattleState>>) {
    println!("战斗开始");
    next_state.set(BattleState::RoundStartPhase);
}

fn handle_battle_round_start_phase(mut next_state: ResMut<NextState<BattleState>>) {
    println!("回合开始阶段");
    next_state.set(BattleState::PlayerInputPhase);
}

fn handle_battle_player_input_phase(
    mut next_state: ResMut<NextState<BattleState>>,
    mut battle_type: MessageReader<BattleType>,
) {
    println!("玩家输入阶段");
    for ele in battle_type.read() {
        match ele {
            BattleType::PvP => println!("玩家对战"),
            BattleType::PvE => println!("玩家对战电脑"),
        }
    }
    next_state.set(BattleState::ActionOrderingPhase);
}

fn handle_battle_action_ordering_phase(mut next_state: ResMut<NextState<BattleState>>) {
    println!("决定出招顺序阶段");
    next_state.set(BattleState::ActionExecutionPhase);
}

fn handle_battle_action_execution_phase(mut next_state: ResMut<NextState<BattleState>>) {
    println!("执行出招阶段");
    next_state.set(BattleState::FaintResolutionPhase);
}

fn handle_battle_faint_resolution_phase(mut next_state: ResMut<NextState<BattleState>>) {
    println!("处理战败和替换阶段");
    next_state.set(BattleState::RoundEndPhase);
}

fn handle_battle_round_end_phase(mut next_state: ResMut<NextState<BattleState>>) {
    println!("回合结束阶段");
    next_state.set(BattleState::End);
}

fn handle_battle_end(mut next_state: ResMut<NextState<BattleState>>) {
    println!("战斗结束");
    next_state.set(BattleState::NoBattle);
}

fn handle_battle_end_exit() {
    println!("战斗结束退出，清理资源");
}
