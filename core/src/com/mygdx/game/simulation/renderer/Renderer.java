package com.mygdx.game.simulation.renderer;

import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.files.FileHandle;
import com.badlogic.gdx.graphics.GL20;
import com.badlogic.gdx.graphics.Pixmap;
import com.badlogic.gdx.graphics.Texture;
import com.badlogic.gdx.graphics.profiling.GLProfiler;
import com.badlogic.gdx.utils.BufferUtils;
import com.badlogic.gdx.utils.Disposable;
import com.mygdx.game.simulation.SimulationObject;
import com.sun.media.jfxmediaimpl.MediaDisposer;
import org.joml.Matrix4f;
import org.joml.Vector3f;
import org.joml.Vector3i;

import java.nio.FloatBuffer;

/**
 * Created by Fran on 3/14/2018.
 */
public class Renderer implements Disposable{

    float total;

    Camera camera;

    Shader shader;

    Transformation transformation;
    private static float FOV =(float)Math.toRadians(45f);
    OpenGLTextureManager openGLTextureManager;

    Model model;

    SimulationObject simulationObject;


    public Renderer() {
        openGLTextureManager = new OpenGLTextureManager();

        new GLProfiler(Gdx.graphics).enable();
        transformation = new Transformation();

        camera = new Camera(0, 0, 0);

        shader = new Shader(Gdx.files.internal("shaders/default.vert"), Gdx.files.internal("shaders/default.frag"));
        model = new Model(openGLTextureManager, "sphere.obj", shader, transformation, new Vector3f(), new Vector3f(), 1);
        simulationObject = new SimulationObject(0,0,0,model);

        shader.use();

        try {
            shader.createUniform("diffuseTex");
            shader.createUniform("projection");
            shader.createUniform("modelView");
        } catch (Exception e) {
            e.printStackTrace();
        }



        Gdx.gl.glEnable(Gdx.gl.GL_DEPTH_TEST);
    }

    public Camera getCamera() {
        return camera;
    }

    public void render(float delta){
        camera.update(delta);
        simulationObject.update(camera);

        Gdx.gl.glClearColor(0.0f,0.0f,0.0f,1.0f);
        Gdx.gl.glClear(Gdx.gl.GL_COLOR_BUFFER_BIT | Gdx.gl.GL_DEPTH_BUFFER_BIT);

        total += delta*50;

        shader.use();
        Matrix4f projection = transformation.getProjectionMatrix(FOV, 800,600,0.01f,1000f);


        shader.setMat4("projection", projection);

        //this.model.render(camera);
        simulationObject.render(camera);
    }



    @Override
    public void dispose() {

    }
}
