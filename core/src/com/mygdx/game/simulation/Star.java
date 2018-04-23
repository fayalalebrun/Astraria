package com.mygdx.game.simulation;

import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.graphics.Color;
import com.mygdx.game.simulation.logic.Body;
import com.mygdx.game.simulation.renderer.*;
import org.joml.Matrix4f;
import org.joml.Vector3d;
import org.joml.Vector3f;
import org.joml.Vector4f;

public class Star extends SphericObject implements LightEmitter{
    private float temperature;
    private final LensGlow lensGlow;
    private final Renderer renderer;
    private final Point occlusionTestPoint;

    private final Vector3f temp3, temp2;

    private final Vector4f temp4f;

    private final Vector3d temp4;

    private final Query query;

    private GLTexture colorSpectrum;

    public Star(Renderer renderer, GLTexture texture, float radius, String name, Transformation transformation, Body body, Color orbitColor, float temperature) {
        super(renderer,texture,renderer.getStarShader(),radius, name, transformation, body,orbitColor);
        this.temperature = temperature;

        this.temp3 = new Vector3f();
        this.temp2 = new Vector3f();

        this.temp4 = new Vector3d();

        this.temp4f = new Vector4f();

        this.renderer = renderer;

        renderer.addLight(this);

        this.lensGlow = new LensGlow(position.x,position.y,position.z,Warehouse.getOpenGLTextureManager().addTexture(Gdx.files.internal("star_glow.png")),
                Warehouse.getOpenGLTextureManager().addTexture(Gdx.files.internal("starspectrum.png")),this,renderer.getTransformation());

        colorSpectrum = Warehouse.getOpenGLTextureManager().addTexture(Gdx.files.internal("star_spectrum_1D.png"));

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
    public void render(Camera cam, float delta) {
        shader.use();
        temp2.set(getPositionRelativeToCamera(cam));
        shader.setVec3f("sunPos", temp2);
        shader.setFloat("temperature", temperature);
        Gdx.gl.glActiveTexture(Gdx.gl.GL_TEXTURE1);
        shader.setInt("sunGradient",1);
        Gdx.gl.glBindTexture(Gdx.gl.GL_TEXTURE_2D, colorSpectrum.ID);


        super.render(cam, delta);





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
    public String toSaveFile() {
        String s = "type: star\n";
        s+= super.toSaveFile();
        s+="temperature: "+temperature+"\n";
        return s;
    }

    @Override
    public void dispose() {
        super.dispose();
        query.dispose();
    }

    @Override
    public Vector3f getAmbient() {
        return temp3.set(0.0f,0.0f,0.0f);
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
