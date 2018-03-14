package com.mygdx.game.simulation;

import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.files.FileHandle;
import com.badlogic.gdx.graphics.Pixmap;
import com.badlogic.gdx.graphics.Texture;
import com.badlogic.gdx.graphics.profiling.GLProfiler;
import com.badlogic.gdx.utils.BufferUtils;
import com.mygdx.game.BaseScreen;
import com.mygdx.game.Boot;
import com.mygdx.game.simulation.renderer.Shader;

import java.nio.ByteBuffer;
import java.nio.ByteOrder;
import java.nio.FloatBuffer;
import java.nio.IntBuffer;

/**
 * Created by fraayala19 on 12/12/17.
 */
public class SimulationScreen extends BaseScreen {

    int VAO;
    Shader shader;
    int texture1;

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


    public SimulationScreen(Boot boot) {
        super(boot);

        new GLProfiler(Gdx.graphics).enable();

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

        texture1 = loadTexture(Gdx.files.internal("particle.png"));

        shader.use();
        shader.setInt("texture1", texture1);
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
    public void show() {


    }


    @Override
    public void render(float delta) {

        Gdx.gl.glClearColor(1f,1f,1f,1.0f);
        Gdx.gl.glClear(Gdx.gl.GL_COLOR_BUFFER_BIT);


        shader.use();
        Gdx.gl30.glBindVertexArray(VAO);
        Gdx.gl.glDrawArrays(Gdx.gl.GL_TRIANGLES, 0, 36);

    }



    @Override
    public void resume() {

    }

    @Override
    public void hide() {

    }

    @Override
    public void pause() {

    }

    @Override
    public void resize(int width, int height) {
    }


    @Override
    public void dispose() {
    }

}
