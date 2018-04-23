package com.mygdx.game.simulation;

import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.graphics.Color;
import com.mygdx.game.simulation.logic.Body;
import com.mygdx.game.simulation.renderer.*;

/**
 * Created by Fran on 4/13/2018.
 */
public class BlackHole extends SimulationObject{

    Skybox skybox;

    public BlackHole(Model model, Shader shader, Shader orbitShader,float radius, String name, Transformation transformation, Body body, Color orbitColor, Skybox skybox) {
        super(model, shader, orbitShader,radius, name, transformation, body, orbitColor);
        this.skybox = skybox;
    }

    @Override
    public void render(Camera cam, float delta) {
        shader.use();
        Gdx.gl.glActiveTexture(Gdx.gl.GL_TEXTURE0);
        shader.setInt("skybox", 0);
        Gdx.gl.glBindTexture(Gdx.gl.GL_TEXTURE_CUBE_MAP, skybox.getCubeMap());
        Gdx.gl.glActiveTexture(Gdx.gl.GL_TEXTURE0);
        shader.setVec3f("holePos", getPositionRelativeToCamera(cam));

        super.render(cam,delta);
    }

    @Override
    public String toSaveFile() {
        String s = "type: black_hole";
        s+="name: "+getName()+"\n";
        s+="radius: "+getRadius()+"\n";
        s+="mass: "+getMass()+"\n";
        s+="velocity: "+getXVel() + " " + getYVel() + " " + getZVel()+"\n";
        s+="position: "+getXPos()+" "+getYPos()+" "+getZPos()+"\n";
        s+="orbit_color: "+orbitColor.r + " " + orbitColor.g + " " + orbitColor.b + " " + orbitColor.a + "\n";
        return s;
    }
}
