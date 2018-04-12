package com.mygdx.game.simulation;

import com.badlogic.gdx.Gdx;
import com.mygdx.game.simulation.renderer.*;
import org.joml.Vector3d;
import org.joml.Vector3f;
import org.joml.Vector4f;

public class Star extends SimulationObject implements LightEmitter{
    private float temperature;
    private final LensGlow lensGlow;
    private final Renderer renderer;
    private final Point occlusionTestPoint;

    private final Vector3f temp3, temp2;

    private final Vector3d temp4;

    private final Query query;

    public Star(double x, double y, double z, Model model, Renderer renderer, float radius, String name, Transformation transformation, float temperature) {
        super(x, y, z, model, renderer.getStarShader(), radius, name, transformation);
        this.temperature = temperature;

        this.temp3 = new Vector3f();
        this.temp2 = new Vector3f();

        this.temp4 = new Vector3d();

        this.renderer = renderer;

        renderer.addLight(this);

        this.lensGlow = new LensGlow(position.x,position.y,position.z,Warehouse.getOpenGLTextureManager().addTexture(Gdx.files.internal("star_glow.png").path()),
                Warehouse.getOpenGLTextureManager().addTexture(Gdx.files.internal("starspectrum.png").path()),this,renderer.getTransformation());

        occlusionTestPoint = new Point(new Vector4f(1,0,0,1),renderer.getTransformation());

        this.query = new Query(Gdx.gl30.GL_ANY_SAMPLES_PASSED);
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





        lensGlow.prepare(renderer,renderer.getLensGlowShader(), renderer.getScreenWidth(), renderer.getScreenHeight(),cam,
                position.x,position.y,position.z);

        renderer.submitLensGlow(lensGlow);



        //doOcclusionTest(cam);


    }

    private void doOcclusionTest(Camera cam){
        if(query.isResultReady()){
            int visibleSamples = query.getResult();
        }


        temp4.set(cam.getPosition());
        temp4.sub(position);
        temp4.normalize();
        temp4.mul(getRadius()*10);
        temp4.add(position);
        occlusionTestPoint.setPosition(temp4.x,temp4.y,temp4.z);



        if(!query.isInUse()) {
            query.start();
            Gdx.gl.glEnable(Gdx.gl.GL_DEPTH_TEST);
            occlusionTestPoint.render(renderer.getPointShader(), cam);
            query.end();
        }
    }

    @Override
    public void dispose() {
        super.dispose();
        query.dispose();
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

    @Override
    public Vector3d getAbsolutePosition() {
        return position;
    }
}
