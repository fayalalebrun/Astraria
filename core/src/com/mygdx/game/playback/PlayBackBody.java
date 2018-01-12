package com.mygdx.game.playback;

import com.badlogic.gdx.graphics.Color;
import com.badlogic.gdx.graphics.g3d.Model;
import com.badlogic.gdx.graphics.g3d.ModelInstance;
import com.badlogic.gdx.math.Vector3;

import java.util.ArrayList;

/**
 * Created by fraayala19 on 1/11/18.
 */
public class PlayBackBody {
    private ArrayList<Vector3> positions;
    private ModelInstance modelInstance;

    public PlayBackBody(Model model) {
        modelInstance = new ModelInstance(model);
    }

    
}
