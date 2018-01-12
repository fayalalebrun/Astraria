package com.mygdx.game.playback;

import com.badlogic.gdx.graphics.Color;
import com.badlogic.gdx.graphics.g3d.Model;
import com.badlogic.gdx.graphics.g3d.ModelInstance;
import com.badlogic.gdx.graphics.g3d.attributes.ColorAttribute;
import com.badlogic.gdx.math.Vector3;

import java.util.ArrayList;

/**
 * Created by fraayala19 on 1/11/18.
 */
public class PlayBackBody {
    private ArrayList<Vector3> positions;
    private ModelInstance modelInstance;

    private Vector3 tempPos;

    public PlayBackBody(Model model) {
        modelInstance = new ModelInstance(model);
        tempPos = new Vector3();
    }

    public void addPosition(Vector3 pos){
        positions.add(pos);
    }

    public void changeColor(Color color){
        modelInstance.materials.get(0).set(ColorAttribute.createDiffuse(color));
    }

    public void setPosition(int index){
        Vector3 posNew = positions.get(index);
        modelInstance.transform.getTranslation(tempPos);
        modelInstance.transform.trn(-tempPos.x, -tempPos.y, -tempPos.z);
        modelInstance.transform.trn(posNew.x, posNew.y, posNew.z);
    }
}
