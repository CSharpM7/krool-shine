use super::super::*;

const DAMAGE:[f32;3] = [3.0,3.0,0.0];
const ANGLE:[u64;3] =  [10,24,361];
const BKB:[i32;3] =  [52,56,90];
const KBG:[i32;3] =  [32,45,46];

const DAMAGE_MOD:[f32;3] = [3.0,3.0,0.0];
const ANGLE_MOD:[u64;3] =  [10,24,361];
const BKB_MOD:[i32;3] =  [52,56,90];
const KBG_MOD:[i32;3] =  [32,45,46];

#[acmd_script( agent = "krool", scripts = ["game_speciallw","game_specialairlw"] , category = ACMD_GAME)]
unsafe fn game_speciallw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;
    let kbg:[i32;2] = if is_HDR() {[KBG_MOD[0],KBG_MOD[1]]} else {[KBG[0],KBG[1]]};
    let bkb:[i32;2] = if is_HDR() {[BKB_MOD[0],BKB_MOD[1]]} else {[BKB[0],BKB[1]]};
    let damage:[f32;2] = if is_HDR() {[DAMAGE_MOD[0],DAMAGE_MOD[1]]} else {[DAMAGE[0],DAMAGE[1]]};
    let angle:[u64;2] = if is_HDR() {[ANGLE_MOD[0],ANGLE_MOD[1]]} else {[ANGLE[0],ANGLE[1]]};
    let hitstop_mul =  if is_HDR() {3.0} else {1.75};

    if is_excute(fighter) {
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);

        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0.0);
        HitModule::set_hit_stop_mul(fighter.module_accessor, 1.0, HitStopMulTarget{_address: (*HIT_STOP_MUL_TARGET_ALL) as u8}, 0.0);
        HitModule::set_hit_stop_mul(fighter.module_accessor, hitstop_mul, HitStopMulTarget{_address: (*HIT_STOP_MUL_TARGET_OPPONENT) as u8}, 0.0);

        //WorkModule::on_flag(boma, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_ON);
        //WorkModule::on_flag(boma, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_ATTACK_DAMAGE_REQUEST);
        //WorkModule::on_flag(boma, *FIGHTER_KROOL_STATUS_WORK_ID_FLAG_WAIST_DAMAGE_REQUEST);

        shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("waist"), 10.0, -1.5, 5.5, 2.0, -1.5, 5.5, 1.0, 1.4, 1.4, 50, false, 1.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT); 
        
        //WorkModule::on_flag(boma, *FIGHTER_KROOL_STATUS_SPECIAL_LW_FLAG_SHIELD);     
        WorkModule::off_flag(boma, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_SPECIAL_EFFECT);
        WorkModule::off_flag(boma, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_HIT_ATTACK);
        WorkModule::off_flag(boma, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_HIT_REFLECT);
    }

    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 1.0/(6.0-1.0));
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_KROOL_STATUS_SPECIAL_LW_FLAG_SHIELD);
    }
    frame(lua_state, 6.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        
        ATTACK(fighter, 0, 0, Hash40::new("top"), damage[0], angle[0], kbg[0], 0, bkb[0], 8.0, 0.0, 9.0, 1.5, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), damage[1], angle[1], kbg[1], 0, bkb[1], 8.0, 0.0, 9.0, 1.5, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        AttackModule::set_add_reaction_frame(boma, 0, 3.0, false);

        // Reflection begins on same frame shine hitbox is active
        ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, 0, 1, 0.54);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter){
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 8.0);
    if is_excute(fighter){
        HitModule::set_hit_stop_mul(fighter.module_accessor, 1.0, HitStopMulTarget{_address: (*HIT_STOP_MUL_TARGET_OPPONENT) as u8}, 0.0);
    }

    frame(lua_state, 29.0);
    if is_excute(fighter){
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    frame(lua_state, 29.0);
    if is_excute(fighter){
        smash_script::shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
    frame(lua_state, 39.0);
    if is_excute(fighter){
        //WorkModule::on_flag(boma, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_OFF);
    }
    frame(lua_state, 43.0);
    FT_MOTION_RATE(fighter, 0.8);
    frame(lua_state, 68.0);
    FT_MOTION_RATE(fighter, 0.5);
    frame(lua_state, 82.0);
    FT_MOTION_RATE(fighter, 1.0);

}

#[acmd_script( agent = "krool",  scripts = ["game_speciallwhit","game_specialairlwhit","game_speciallwturn","game_specialairlwturn"], category = ACMD_GAME )]
unsafe fn game_speciallwhit(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;
    //Use air params for no counter, else use counter params
    let kbg  = if is_HDR() {KBG_MOD[2]} else {KBG[2]};
    let bkb = if is_HDR() {BKB_MOD[2]} else {BKB[2]};
    let damage = if is_HDR() {DAMAGE_MOD[2]} else {DAMAGE[2]};
    let angle = if is_HDR() {ANGLE_MOD[2]} else {ANGLE[2]};
    
    let useCounterParams = false;
    let i = if useCounterParams {1} else {0};

    let mut turnOffset = 0.0;
    let mut isTurn = false;
    let mut damage_received = 0.0;

    if is_excute(fighter) {
        if WorkModule::is_flag(boma, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_HIT_REFLECT)
        || WorkModule::is_flag(boma, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_HIT_ATTACK) {
            WorkModule::on_flag(boma, *FIGHTER_KROOL_STATUS_SPECIAL_LW_FLAG_TURN);
        }

        shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("waist"), 10.0, -1.5, 5.5, 2.0, -1.5, 5.5, 1.0, 1.4, 1.4, 50, false, 1.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);   
        damage_received = WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_SUCCEED_HIT_DAMAGE);

        if WorkModule::is_flag(boma, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_HIT_REFLECT)
        || WorkModule::is_flag(boma, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_HIT_ATTACK) {
            WHOLE_HIT(fighter, *HIT_STATUS_INVINCIBLE);
        }
        else{
            WorkModule::on_flag(boma, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_ON);
            WorkModule::on_flag(boma, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_ATTACK_DAMAGE_REQUEST);
        }

    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_KROOL_STATUS_SPECIAL_LW_FLAG_TURN);
        if ControlModule::get_stick_x(boma).signum() != PostureModule::lr(boma)
        && ControlModule::get_stick_x(boma).abs() >= 0.5
        {
            isTurn=true;
            turnOffset = 5.0;
        }
    }
    FT_MOTION_RATE(fighter, (1.0+turnOffset)/((3.0+turnOffset)-1.0));

    frame(lua_state, 2.0 + turnOffset);
    if is_excute(fighter) {
        /*
        if WorkModule::is_flag(boma, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_HIT_REFLECT)
        || WorkModule::is_flag(boma, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_HIT_ATTACK) {
            WorkModule::on_flag(boma, *FIGHTER_KROOL_STATUS_SPECIAL_LW_FLAG_TURN);
        } */
    }
    frame(lua_state, 3.0 + turnOffset);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        let totalDamage = (damage_received*0.7).min(50.0);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, angle, kbg, 0, bkb, 12.0, 0.0, 12.5, 8.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        AttackModule::set_force_reaction(boma, 0, true, false);

        if WorkModule::is_flag(boma, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_HIT_REFLECT)
        || WorkModule::is_flag(boma, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_HIT_ATTACK) 
        {
            ATTACK(fighter, 1, 0, Hash40::new("top"), 0.0, angle, kbg, 0, bkb, 14.0, 0.0, 13.0, 12.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            AttackModule::set_force_reaction(boma, 1, true, false);
        }

    }
    frame(lua_state, 10.0 + turnOffset);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        WorkModule::on_flag(boma, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_OFF);
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        WHOLE_HIT(fighter, *HIT_STATUS_NORMAL);
    }
    frame(lua_state, 18.0 + turnOffset);
    FT_MOTION_RATE(fighter, 0.7);
    frame(lua_state, 48.0 + turnOffset);
    FT_MOTION_RATE(fighter, 1.0);
}

#[acmd_script( agent = "krool", scripts = ["sound_speciallwhit","sound_specialairlwhit","sound_speciallwturn","sound_specialairlwturn"], category = ACMD_SOUND )]
unsafe fn sound_speciallwhit(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;
    let mut turnOffset = 0.0;
    let mut isTurn = false;
    
    if is_excute(fighter) {
        if MotionModule::motion_kind(boma) == Hash40::new("special_lw_turn").hash || MotionModule::motion_kind(boma) == Hash40::new("special_air_lw_turn").hash
        {
            isTurn=true;
            turnOffset = 9.0;
        }
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) && !isTurn {
        PLAY_SE(fighter, Hash40::new("se_krool_special_l02"));
        if WorkModule::is_flag(boma, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_HIT_REFLECT)
        || WorkModule::is_flag(boma, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_HIT_ATTACK)
        {
            PLAY_SE(fighter, Hash40::new("vc_krool_special_l03"));
        }
    }
    frame(lua_state, turnOffset);
    if is_excute(fighter) && isTurn {
        PLAY_SE(fighter, Hash40::new("se_krool_special_l02"));
        if WorkModule::is_flag(boma, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_HIT_REFLECT)
        || WorkModule::is_flag(boma, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_HIT_ATTACK)
        {
            PLAY_SE(fighter, Hash40::new("vc_krool_special_l03"));
        }
    }
}
#[acmd_script( agent = "krool", scripts = ["effect_speciallwhit","effect_specialairlwhit","effect_speciallwturn","effect_specialairlwturn"], category = ACMD_EFFECT, low_priority )]
unsafe fn effect_speciallwhit(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;
    let lr = PostureModule::lr(boma);

    let mut turnOffset = 0.0;
    let mut turnRot=0.0;
    let mut turnLR = 1.0;
    let mut isTurn = false;

    if is_excute(fighter) {
        if MotionModule::motion_kind(boma) == Hash40::new("special_lw_turn").hash || MotionModule::motion_kind(boma) == Hash40::new("special_air_lw_turn").hash
        {
            isTurn=true;
            turnOffset = 6.0;
            turnRot = 180.0;
            turnLR = -1.0;
        }
    }

    frame(lua_state, 1.0+turnOffset);
    if is_excute(fighter) {
        if isTurn{
            EFFECT_OFF_KIND(fighter, Hash40::new("sys_h_smoke_b"), true, true);
            wait(lua_state, 1.0);
        }
        else{
            LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        EFFECT(fighter, Hash40::new("krool_counter_success"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);

        EFFECT_OFF_KIND(fighter, Hash40::new("krool_counter_flash"), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("krool_counter_flash_l"), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_guard_mark"), true, true);

        let eff = if (lr < 0.0) {"krool_counter_success_body_l"} else {"krool_counter_success_body"};
        EFFECT(fighter, Hash40::new(eff), Hash40::new("top"), 0, 0, 0, 0, turnRot, 0, 1, 0, 0, 0, 0, 0, 0, true);

        if(WorkModule::is_flag(boma, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_SPECIAL_EFFECT)){
            EFFECT(fighter, Hash40::new("sys_counter_flash"), Hash40::new("top"), 0, 9, 8, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(lua_state, 2.0+turnOffset);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("krool_counter_success_body"), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("krool_counter_success_body_l"), true, true);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
        let eff = if (lr < 0.0) {"krool_counter_attack_body_l"} else {"krool_counter_attack_body"};
        EFFECT_FOLLOW(fighter, Hash40::new(eff), Hash40::new("top"), 0, 0, 0, 0, turnRot, 0, 1, true);

        if WorkModule::is_flag(boma, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_HIT_REFLECT)
        || WorkModule::is_flag(boma, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_HIT_ATTACK)
        {
            EFFECT(fighter, Hash40::new("krool_counter_attack"), Hash40::new("top"), 6.0*lr*turnLR, 14, 16.0*turnLR, -90, (turnRot-30.0)*lr, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
            EFFECT(fighter, Hash40::new("krool_counter_attack2"), Hash40::new("top"), 6.0*lr*turnLR, 12, 22.0*turnLR, 80, (turnRot-25.0)*lr, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        else
        {
            EFFECT(fighter, Hash40::new("krool_counter_attack"), Hash40::new("top"), 6.0*lr, 12, 14, -90, 30.0*lr, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
        }
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("krool_counter_attack2"), false, true);
    }
}

pub fn install() {
    install_acmd_scripts!(
        game_speciallw,
        game_speciallwhit,
        sound_speciallwhit,
        effect_speciallwhit,
    );
}