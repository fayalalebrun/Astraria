package com.mygdx.game.simulation.renderer;

import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.files.FileHandle;
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


    int VAO;
    Shader shader;
    int texture1;
    float total;
    Transformation transformation;
    private static float FOV =(float)Math.toRadians(45f);

    float vertices[] = {
            -0.5f, -0.5f, -0.5f,  0.0f, 0.0f,
            0.5f, -0.5f, -0.5f,  1.0f, 0.0f,
            0.5f,  0.5f, -0.5f,  1.0f, 1.0f,
            0.5f,  0.5f, -0.5f,  1.0f, 1.0f,
            -0.5f,  0.5f, -0.5f,  0.0f, 1.0f,
            -0.5f, -0.5f, -0.5f,  0.0f, 0.0f,

            -0.5f, -0.5f,  0.5f,  0.0f, 0.0f,
            0.5f, -0.5f,  0.5f,  1.0f, 0.0f,
            0.5f,  0.5f,  0.5f,  1.0f, 1.0f,
            0.5f,  0.5f,  0.5f,  1.0f, 1.0f,
            -0.5f,  0.5f,  0.5f,  0.0f, 1.0f,
            -0.5f, -0.5f,  0.5f,  0.0f, 0.0f,

            -0.5f,  0.5f,  0.5f,  1.0f, 0.0f,
            -0.5f,  0.5f, -0.5f,  1.0f, 1.0f,
            -0.5f, -0.5f, -0.5f,  0.0f, 1.0f,
            -0.5f, -0.5f, -0.5f,  0.0f, 1.0f,
            -0.5f, -0.5f,  0.5f,  0.0f, 0.0f,
            -0.5f,  0.5f,  0.5f,  1.0f, 0.0f,

            0.5f,  0.5f,  0.5f,  1.0f, 0.0f,
            0.5f,  0.5f, -0.5f,  1.0f, 1.0f,
            0.5f, -0.5f, -0.5f,  0.0f, 1.0f,
            0.5f, -0.5f, -0.5f,  0.0f, 1.0f,
            0.5f, -0.5f,  0.5f,  0.0f, 0.0f,
            0.5f,  0.5f,  0.5f,  1.0f, 0.0f,

            -0.5f, -0.5f, -0.5f,  0.0f, 1.0f,
            0.5f, -0.5f, -0.5f,  1.0f, 1.0f,
            0.5f, -0.5f,  0.5f,  1.0f, 0.0f,
            0.5f, -0.5f,  0.5f,  1.0f, 0.0f,
            -0.5f, -0.5f,  0.5f,  0.0f, 0.0f,
            -0.5f, -0.5f, -0.5f,  0.0f, 1.0f,

            -0.5f,  0.5f, -0.5f,  0.0f, 1.0f,
            0.5f,  0.5f, -0.5f,  1.0f, 1.0f,
            0.5f,  0.5f,  0.5f,  1.0f, 0.0f,
            0.5f,  0.5f,  0.5f,  1.0f, 0.0f,
            -0.5f,  0.5f,  0.5f,  0.0f, 0.0f,
            -0.5f,  0.5f, -0.5f,  0.0f, 1.0f
    };

    public Renderer() {
        new GLProfiler(Gdx.graphics).enable();
        transformation = new Transformation();

        shader = new Shader(Gdx.files.internal("shaders/default.vert"), Gdx.files.internal("shaders/default.frag"));


        int[] tempv = new int[1];

        Gdx.gl30.glGenVertexArrays(1, tempv, 0);

        VAO = tempv[0];
        int VBO = Gdx.gl.glGenBuffer();

        Gdx.gl30.glBindVertexArray(VAO);

        Gdx.gl.glBindBuffer(Gdx.gl.GL_ARRAY_BUFFER, VBO);
        FloatBuffer vertexData = BufferUtils.newFloatBuffer(vertices.length*3);
        vertexData.put(vertices);
        vertexData.flip();
        Gdx.gl.glBufferData(Gdx.gl.GL_ARRAY_BUFFER, vertices.length*4,vertexData, Gdx.gl.GL_STATIC_DRAW);

        Gdx.gl.glVertexAttribPointer(0, 3, Gdx.gl.GL_FLOAT, false,20, 0);
        Gdx.gl.glEnableVertexAttribArray(0);
        Gdx.gl.glVertexAttribPointer(1, 2, Gdx.gl.GL_FLOAT, false, 20, 12);
        Gdx.gl.glEnableVertexAttribArray(1);

        texture1 = loadTexture(Gdx.files.internal("container.jpg"));

        shader.use();

        try {
            shader.createUniform("texture1");
            shader.createUniform("projection");
            shader.createUniform("view");
            shader.createUniform("model");
        } catch (Exception e) {
            e.printStackTrace();
        }

        shader.setInt("texture1", texture1);


        Gdx.gl.glEnable(Gdx.gl.GL_DEPTH_TEST);
    }

    public void render(float delta){
        total+=delta*50;




        Gdx.gl.glActiveTexture(Gdx.gl.GL_TEXTURE0);
        Gdx.gl.glBindTexture(Gdx.gl.GL_TEXTURE_2D, texture1);

        shader.use();
        Matrix4f projection = transformation.getProjectionMatrix(FOV, 800,600,0.01f,1000f);
        Matrix4f view = new Matrix4f().translate(0f,0f,-3.0f);
        Matrix4f model = transformation.getWorldMatrix(new Vector3f(),new Vector3f(total*(float)Math.toRadians(50f),total*(float)Math.toRadians(50f),0),1f);

        shader.setMat4("projection", projection);
        shader.setMat4("view", view);
        shader.setMat4("model", model);

        Gdx.gl30.glBindVertexArray(VAO);
        Gdx.gl.glDrawArrays(Gdx.gl.GL_TRIANGLES, 0, 36);
    }

    private int loadTexture(FileHandle handle){
        int texture = Gdx.gl.glGenTexture();
        Gdx.gl.glBindTexture(Gdx.gl.GL_TEXTURE_2D, texture);

        Gdx.gl.glTexParameteri(Gdx.gl.GL_TEXTURE_2D, Gdx.gl.GL_TEXTURE_WRAP_S, Gdx.gl.GL_REPEAT);
        Gdx.gl.glTexParameteri(Gdx.gl.GL_TEXTURE_2D, Gdx.gl.GL_TEXTURE_WRAP_T, Gdx.gl.GL_REPEAT);

        Gdx.gl.glTexParameteri(Gdx.gl.GL_TEXTURE_2D, Gdx.gl.GL_TEXTURE_MIN_FILTER, Gdx.gl.GL_LINEAR);
        Gdx.gl.glTexParameteri(Gdx.gl.GL_TEXTURE_2D, Gdx.gl.GL_TEXTURE_MAG_FILTER, Gdx.gl.GL_LINEAR);

        Texture tex = new Texture(handle);

        tex.getTextureData().prepare();


        Pixmap pixmap = tex.getTextureData().consumePixmap();

        Gdx.gl.glTexImage2D(Gdx.gl.GL_TEXTURE_2D, 0, pixmap.getGLInternalFormat(), pixmap.getWidth(),
                pixmap.getHeight(), 0, pixmap.getGLFormat(), pixmap.getGLType(),
                pixmap.getPixels());
        Gdx.gl.glGenerateMipmap(Gdx.gl.GL_TEXTURE_2D);
        pixmap.dispose();
        tex.dispose();
        return texture;
    }

    @Override
    public void dispose() {

    }
}
