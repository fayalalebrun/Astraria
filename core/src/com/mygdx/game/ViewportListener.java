package com.mygdx.game;

import com.badlogic.gdx.graphics.Camera;
import com.badlogic.gdx.graphics.g3d.utils.FirstPersonCameraController;

/**
 * Created by fraayala19 on 12/18/17.
 */
public class ViewportListener extends FirstPersonCameraController{

    private float scrollAmount;

    public ViewportListener(Camera camera) {
        super(camera);
    }

    @Override
    public boolean scrolled(int amount) {
        scrollAmount+=amount;
        float vel = scrollAmount;
        if(scrollAmount>=0) {
            vel = scrollAmount;
        } else if (scrollAmount<0){
            vel = (1/scrollAmount);
        }
        vel = (float)Math.pow(vel,1.5);
        vel*=0.000001f;
        System.out.println(vel);
        setVelocity(vel);
        return super.scrolled(amount);
    }

}
