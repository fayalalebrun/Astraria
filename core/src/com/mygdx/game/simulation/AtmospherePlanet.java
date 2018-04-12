package com.mygdx.game.simulation;

import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.graphics.Color;
import com.mygdx.game.simulation.logic.Body;
import com.mygdx.game.simulation.renderer.*;
import org.joml.Vector3f;
import org.joml.Vector4f;

/**
 * Created by Fran on 4/10/2018.
 */
public class AtmospherePlanet extends SimulationObject{

    private int atmosphereMapTexture;

    private LightSourceManager lightSourceManager;

    private final Vector3f starPos, temp;
    private final Vector4f temp4f;
    private Color atmosphereColor;

    public AtmospherePlanet( Model model, Shader shader, float radius, String name, Transformation transformation, Body body,
                             LightSourceManager lightSourceManager, Color atmosphereColor) {
        super(model, shader, radius, name, transformation, body);
        this.atmosphereMapTexture = Warehouse.getOpenGLTextureManager().addTexture(Gdx.files.internal("atmoGradient.png").path());
        this.lightSourceManager = lightSourceManager;

        starPos = new Vector3f();
        temp = new Vector3f();
        temp4f = new Vector4f();
        this.atmosphereColor = new Color().set(atmosphereColor);
    }

    @Override
    protected void update(Camera cam) {
        super.update(cam);

    }


    private void setUniforms(Camera cam){

        shader.use();

        shader.setVec3f("star_pos",starPos.set(lightSourceManager.getTopAbsPos()).sub(temp.set(cam.getPosition())));
        shader.setVec3f("planet_pos",getPositionRelativeToCamera(cam));
        shader.setVec4f("atmoColorMod", temp4f.set(atmosphereColor.r,atmosphereColor.g,atmosphereColor.b,atmosphereColor.a));


        Gdx.gl.glActiveTexture(Gdx.gl.GL_TEXTURE1);
        shader.setInt("txratm",1);
        Gdx.gl.glBindTexture(Gdx.gl.GL_TEXTURE_2D, atmosphereMapTexture);

        Gdx.gl.glActiveTexture(Gdx.gl.GL_TEXTURE0);
    }

    @Override
    public void render(Camera cam) {
        setUniforms(cam);

        super.render(cam);

    }

    @Override
    public void dispose() {
        super.dispose();
    }
}
