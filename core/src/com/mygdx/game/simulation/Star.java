package com.mygdx.game.simulation;

import com.badlogic.gdx.Gdx;
import com.mygdx.game.simulation.renderer.*;
import org.joml.Vector3f;

public class Star extends SimulationObject implements LightEmitter{
    private float temperature;
    private LensGlow lensGlow;
    private final Renderer renderer;

    Vector3f temp3, temp2;

    public Star(double x, double y, double z, Model model, Renderer renderer, float size, String name, float temperature) {
        super(x, y, z, model, renderer.getStarShader(), size, name);
        this.temperature = temperature;

        this.temp3 = new Vector3f();
        this.temp2 = new Vector3f();

        this.renderer = renderer;

        renderer.addLight(this);

        this.lensGlow = new LensGlow(position.x,position.y,position.z,renderer.getOpenGLTextureManager().addTexture(Gdx.files.internal("star_glow.png").path()),
                renderer.getOpenGLTextureManager().addTexture(Gdx.files.internal("starspectrum.png").path()),this,renderer.getTransformation());
    }

    @Override
    public Vector3f getLightPosition(Camera camera, Transformation transformation) {
        return transformation.getViewMatrix(camera).transformProject(temp3.set(position).sub(temp2.set(camera.getPosition())));
    }

    public float getTemperature() {
        return temperature;
    }

    @Override
    public void render(Camera cam) {
        super.render(cam);

        lensGlow.setPosition(position.x,position.y,position.z);

        lensGlow.render(renderer.getLensGlowShader(),renderer.getScreenWidth(),renderer.getScreenHeight(), cam);
    }

    @Override
    public Vector3f getAmbient() {
        return temp3.set(0.05f,0.05f,0.05f);
    }

    @Override
    public Vector3f getDiffuse() {
        return temp3.set(0.8f,0.8f,0.8f);
    }

    @Override
    public Vector3f getSpecular() {
        return temp3.set(1.0f,1.0f,1.0f);
    }
}
