package com.mygdx.game.simulation;

import com.badlogic.gdx.Gdx;
import com.mygdx.game.simulation.renderer.*;
import org.joml.Vector3f;
import org.joml.Vector4f;
import sun.security.provider.SHA;

/**
 * Created by Fran on 4/10/2018.
 */
public class AtmospherePlanet extends SimulationObject{

    private int atmosphereMapTexture;

    private LightSourceManager lightSourceManager;

    private final Vector3f starPos, temp;
    private final Vector4f temp4f;

    public AtmospherePlanet(double x, double y, double z, Model model, Shader shader, float radius, String name, Transformation transformation, int atmosphereMapTexture,LightSourceManager lightSourceManager) {
        super(x, y, z, model, shader, radius, name, transformation);
        this.atmosphereMapTexture = atmosphereMapTexture;
        this.lightSourceManager = lightSourceManager;

        starPos = new Vector3f();
        temp = new Vector3f();
        temp4f = new Vector4f();

    }

    @Override
    protected void update(Camera cam) {
        super.update(cam);

    }


    private void setUniforms(Camera cam){

        shader.use();

        shader.setVec3f("star_pos",starPos.set(lightSourceManager.getTop().getAbsolutePosition()).sub(temp.set(cam.getPosition())));
        shader.setVec3f("planet_pos",getPositionRelativeToCamera(cam));


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
