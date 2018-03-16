package com.mygdx.game.simulation.renderer;

import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.files.FileHandle;
import com.badlogic.gdx.graphics.GL20;
import com.badlogic.gdx.graphics.Pixmap;
import com.badlogic.gdx.graphics.Texture;
import com.badlogic.gdx.graphics.profiling.GLProfiler;
import com.badlogic.gdx.utils.BufferUtils;
import com.badlogic.gdx.utils.Disposable;
import com.sun.media.jfxmediaimpl.MediaDisposer;
import org.joml.Matrix4f;
import org.joml.Vector3f;

import java.nio.FloatBuffer;

/**
 * Created by Fran on 3/14/2018.
 */
public class Renderer implements Disposable{



    Shader shader;

    Transformation transformation;
    private static float FOV =(float)Math.toRadians(45f);
    OpenGLTextureManager openGLTextureManager;

    Model model;


    public Renderer() {
        openGLTextureManager = new OpenGLTextureManager();

        new GLProfiler(Gdx.graphics).enable();
        transformation = new Transformation();

        shader = new Shader(Gdx.files.internal("shaders/default.vert"), Gdx.files.internal("shaders/default.frag"));
        model = new Model(openGLTextureManager, "box.obj", shader);



        shader.use();

        try {
            shader.createUniform("diffuseTex");
            shader.createUniform("projection");
            shader.createUniform("view");
            shader.createUniform("model");
        } catch (Exception e) {
            e.printStackTrace();
        }



        Gdx.gl.glEnable(Gdx.gl.GL_DEPTH_TEST);
    }

    public void render(float delta){
        Gdx.gl.glClear(Gdx.gl.GL_COLOR_BUFFER_BIT | Gdx.gl.GL_DEPTH_BUFFER_BIT);

        shader.use();
        Matrix4f projection = transformation.getProjectionMatrix(FOV, 800,600,0.01f,1000f);
        Matrix4f view = new Matrix4f().translate(0f,0f,-3.0f);
        Matrix4f model = new Matrix4f().identity();


        shader.setMat4("projection", projection);
        shader.setMat4("view", view);
        shader.setMat4("model", model);

        this.model.render(model);

    }



    @Override
    public void dispose() {

    }
}
