use super::*;

// Fox Shine Jump Cancels
unsafe fn shine_jump_cancel(fighter: &mut L2CFighterCommon,boma: &mut BattleObjectModuleAccessor, status: i32) {
    if status == *FIGHTER_STATUS_KIND_SPECIAL_LW && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_FRAME_IN_AIR) <= 1 {
        GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }
    let minFrame = 5.0;
    let maxFrame = 29.0;
    if (status == *FIGHTER_STATUS_KIND_SPECIAL_LW )
    {
        HIT_NODE(fighter, Hash40::new("waist"), *HIT_STATUS_NORMAL);
        if (MotionModule::frame(boma) >= minFrame
        && MotionModule::frame(boma) <= maxFrame)
        {    
            if (WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME) > 0.0
            || StopModule::is_stop(boma))
            {
                WorkModule::on_flag(boma, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_HIT_ATTACK);
                WorkModule::on_flag(boma, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_SPECIAL_EFFECT);
            }
            else if AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT){
                WorkModule::on_flag(boma, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_HIT_ATTACK);
            }

            if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_ATTACK) 
            && MotionModule::frame(boma) >= minFrame+2.0{
                StatusModule::change_status_request(boma, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_LW_HIT, false);
            }
            else if !is_in_hitlag(boma)
            {
                check_jump_cancel(fighter,false);
            }
        }
    }
}   


#[fighter_frame( agent = FIGHTER_KIND_KROOL )]
fn krool_update(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = StatusModule::status_kind(boma);
        let motion_kind = MotionModule::motion_kind(boma);

        //shine_jump_cancel(fighter,boma,status_kind);
    }
}


pub fn install() {
    smashline::install_agent_frames!(
        krool_update
    );
}