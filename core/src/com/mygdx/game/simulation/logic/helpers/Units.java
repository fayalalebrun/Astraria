package com.mygdx.game.simulation.logic.helpers;

/**
 * Created by fraayala19 on 12/13/17.
 */
public class Units {
    public static final double GRAV = 6.67408 * Math.pow(10,-11);

    public static double mToAU(double meters){
        return meters/149597870691.0;
    }

    public static double AUToM(double au){
        return au*149597870691.0;
    }
}
