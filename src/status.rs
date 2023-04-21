use super::*;

#[status_script(agent = "krool", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_lw_main_motion_helper(fighter);
    return fighter.sub_shift_status_main(L2CValue::Ptr(special_lw_main_loop as *const () as _));
}

unsafe extern "C" fn special_lw_main_motion_helper(fighter: &mut L2CFighterCommon) {
    
    let motion_g = "special_lw";
    let motion_a = "special_air_lw";
    let situation = StatusModule::situation_kind(fighter.module_accessor);
    if situation != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new(motion_a), -1.0, 1.0, 0.0);
    } 
    else {
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, ENERGY_MOTION_RESET_TYPE_GROUND_TRANS, 0.0, 0.0, 0.0, 0.0, 0.0);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_IGNORE_NORMAL);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new(motion_g), -1.0, 1.0, 0.0);
    }

}

unsafe extern "C" fn special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }

    let minFrame = 5.0;
    let maxFrame = 29.0;

    if !StatusModule::is_changing(fighter.module_accessor) {
        if (MotionModule::frame(fighter.module_accessor) >= minFrame
        && MotionModule::frame(fighter.module_accessor) <= maxFrame)
        {    
            if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) 
            && MotionModule::frame(fighter.module_accessor) >= minFrame+2.0 {
                StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_LW_HIT, false);
            }
            else if !is_in_hitlag(fighter.module_accessor)
            {
                check_jump_cancel(fighter,false);
            }
        }
        if StatusModule::is_situation_changed(fighter.module_accessor) {
            special_lw_main_motion_helper(fighter);
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    0.into()
}

#[status_script(agent = "krool", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
pub unsafe fn special_lw_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let status = StatusModule::status_kind(boma);

    if status == *FIGHTER_STATUS_KIND_SPECIAL_LW && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FRAME_IN_AIR) <= 1 {
        GroundModule::correct(boma, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }
    let minFrame = 5.0;
    let maxFrame = if WorkModule::is_flag(boma, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_HIT_ATTACK) {34.0} else {29.0};
    if (status == *FIGHTER_STATUS_KIND_SPECIAL_LW )
    {
        HitModule::set_status_all(boma, HitStatus(*HIT_STATUS_NORMAL), 0);
        WorkModule::on_flag(boma, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_ATTACK_DAMAGE_REQUEST);
        let damage_received = WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_SUCCEED_HIT_DAMAGE);

        if (WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME) > 0.0
        || FighterStopModuleImpl::is_damage_stop(boma)
        || damage_received > 0.0)
        {
            WorkModule::on_flag(boma, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_HIT_ATTACK);
            //WorkModule::on_flag(boma, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_SPECIAL_EFFECT);
        }
        else if AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT){
            //WorkModule::on_flag(boma, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_HIT_ATTACK);
        }
        if (MotionModule::frame(boma) >= minFrame
        && MotionModule::frame(boma) <= maxFrame)
        {    
            if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_ATTACK) 
            && MotionModule::frame(boma) >= minFrame+2.0{
                StatusModule::change_status_request(boma, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_LW_HIT, false);
                return 1.into()
            }
            else if !is_in_hitlag(boma)
            {
                check_jump_cancel(fighter,false);
            }
        }
    }

    return 0.into()
}
pub fn install() {
    install_status_scripts!(
        //special_lw_main
        special_lw_exec
    );
}