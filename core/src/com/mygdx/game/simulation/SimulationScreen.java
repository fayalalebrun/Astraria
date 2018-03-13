package com.mygdx.game.simulation;

import com.badlogic.gdx.Gdx;
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

    float vertices[] = {
            -0.5f, -0.5f, 0.0f,
            0.5f, -0.5f, 0.0f,
            0.0f,  0.5f, 0.0f
    };


    public SimulationScreen(Boot boot) {
        super(boot);

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

        Gdx.gl.glVertexAttribPointer(0, 3, Gdx.gl.GL_FLOAT, false,12, 0);
        Gdx.gl.glEnableVertexAttribArray(0);

    }

    @Override
    public void show() {


    }


    @Override
    public void render(float delta) {

        Gdx.gl.glClearColor(0.2f,0.0f,0.3f,1.0f);
        Gdx.gl.glClear(Gdx.gl.GL_COLOR_BUFFER_BIT);


        shader.use();
        Gdx.gl30.glBindVertexArray(VAO);
        Gdx.gl.glDrawArrays(Gdx.gl.GL_TRIANGLES, 0, 3);

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
