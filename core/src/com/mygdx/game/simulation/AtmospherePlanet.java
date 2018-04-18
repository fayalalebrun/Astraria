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

    private boolean useAmbientTexture;

    private int ambientTexture;

    public AtmospherePlanet( Model model, Shader shader, Shader orbitShader, float radius, String name, Color orbitColor,
                             Transformation transformation, Body body,
                             LightSourceManager lightSourceManager, Color atmosphereColor, int ambientTexture) {
        super(model, shader, orbitShader, radius, name, transformation, body, orbitColor);
        this.atmosphereMapTexture = Warehouse.getOpenGLTextureManager().addTexture(Gdx.files.internal("atmoGradient.png").path());
        this.lightSourceManager = lightSourceManager;

        starPos = new Vector3f();
        temp = new Vector3f();
        temp4f = new Vector4f();
        this.atmosphereColor = new Color().set(atmosphereColor);

        this.ambientTexture = ambientTexture;

        if(ambientTexture!=-1){
            useAmbientTexture = true;
        }
    }


    private void setUniforms(Camera cam){

        shader.use();

        shader.setVec3f("star_pos",starPos.set(lightSourceManager.getTopAbsPos()).sub(temp.set(cam.getPosition())));
        shader.setVec3f("planet_pos",getPositionRelativeToCamera(cam));
        shader.setVec4f("atmoColorMod", temp4f.set(atmosphereColor.r,atmosphereColor.g,atmosphereColor.b,atmosphereColor.a));

        if (useAmbientTexture) {
            shader.setInt("useAmbTex", 1);
            Gdx.gl.glActiveTexture(Gdx.gl.GL_TEXTURE5);
            shader.setInt("ambTex", 5);
            Gdx.gl.glBindTexture(Gdx.gl.GL_TEXTURE_2D, ambientTexture);
            Gdx.gl.glActiveTexture(Gdx.gl.GL_TEXTURE0);
        } else {
            shader.setInt("useAmbTex", 0);
        }

        Gdx.gl.glActiveTexture(Gdx.gl.GL_TEXTURE1);
        shader.setInt("txratm",1);
        Gdx.gl.glBindTexture(Gdx.gl.GL_TEXTURE_2D, atmosphereMapTexture);

        Gdx.gl.glActiveTexture(Gdx.gl.GL_TEXTURE0);
    }

    @Override
    public void render(Camera cam, float delta) {
        setUniforms(cam);

        super.render(cam, delta);

    }

    @Override
    public void dispose() {
        super.dispose();
    }
}
