package com.mygdx.game.simulation.renderer;

import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.utils.BufferUtils;
import com.badlogic.gdx.utils.Disposable;

import java.nio.FloatBuffer;

import static com.badlogic.gdx.graphics.GL20.GL_ARRAY_BUFFER;

/**
 * Created by fraayala19 on 4/13/18.
 */
public class Skybox implements Disposable{
    protected float[] vertices;

    private int VAO;
    private int VBO;

    private int cubeMap;

    private Shader shader;

    public Skybox(int cubeMap, Shader shader) {
        this.cubeMap = cubeMap;

        vertices = new float[]{
                // positions
                -10f,  10f, -10f,
                -10f, -10f, -10f,
                10f, -10f, -10f,
                10f, -10f, -10f,
                10f,  10f, -10f,
                -10f,  10f, -10f,

                -10f, -10f,  10f,
                -10f, -10f, -10f,
                -10f,  10f, -10f,
                -10f,  10f, -10f,
                -10f,  10f,  10f,
                -10f, -10f,  10f,

                10f, -10f, -10f,
                10f, -10f,  10f,
                10f,  10f,  10f,
                10f,  10f,  10f,
                10f,  10f, -10f,
                10f, -10f, -10f,

                -10f, -10f,  10f,
                -10f,  10f,  10f,
                10f,  10f,  10f,
                10f,  10f,  10f,
                10f, -10f,  10f,
                -10f, -10f,  10f,

                -10f,  10f, -10f,
                10f,  10f, -10f,
                10f,  10f,  10f,
                10f,  10f,  10f,
                -10f,  10f,  10f,
                -10f,  10f, -10f,

                -10f, -10f, -10f,
                -10f, -10f,  10f,
                10f, -10f, -10f,
                10f, -10f, -10f,
                -10f, -10f,  10f,
                10f, -10f,  10f
        };

        this.shader = shader;

        setupMesh();
    }


    private void setupMesh(){
        int[] tempv = new int[1];
        Gdx.gl30.glGenVertexArrays(1, tempv, 0);
        VAO = tempv[0];

        Gdx.gl30.glBindVertexArray(VAO);

        VBO = Gdx.gl.glGenBuffer();
        Gdx.gl.glBindBuffer(GL_ARRAY_BUFFER, VBO);
        Gdx.gl.glBufferData(GL_ARRAY_BUFFER, vertices.length*4, toFloatBuffer(vertices), Gdx.gl.GL_STATIC_DRAW);
        Gdx.gl.glEnableVertexAttribArray(0);
        Gdx.gl.glVertexAttribPointer(0, 3, Gdx.gl.GL_FLOAT, false, 0, 0);

        Gdx.gl.glDisableVertexAttribArray(0);
        Gdx.gl.glBindBuffer(GL_ARRAY_BUFFER, 0);
        Gdx.gl30.glBindVertexArray(0);
    }

    public void render(){
        Gdx.gl.glDisable(Gdx.gl.GL_BLEND);
        Gdx.gl.glDepthMask(false);

        Gdx.gl.glDepthFunc(Gdx.gl.GL_LEQUAL);
        shader.use();
        Gdx.gl.glActiveTexture(Gdx.gl.GL_TEXTURE0);
        shader.setInt("skybox", 0);
        Gdx.gl.glBindTexture(Gdx.gl.GL_TEXTURE_CUBE_MAP, cubeMap);
        Gdx.gl.glActiveTexture(Gdx.gl.GL_TEXTURE0);

        Gdx.gl30.glBindVertexArray(VAO);

        Gdx.gl.glEnableVertexAttribArray(0);

        Gdx.gl.glDrawArrays(Gdx.gl.GL_TRIANGLES, 0, 36);

        Gdx.gl.glDisableVertexAttribArray(0);
        Gdx.gl30.glBindVertexArray(0);

        Gdx.gl.glDepthMask(true);
        Gdx.gl.glEnable(Gdx.gl.GL_BLEND);
        Gdx.gl.glDepthFunc(Gdx.gl.GL_LESS);
    }

    protected FloatBuffer toFloatBuffer(float[] arr){
        FloatBuffer buff = BufferUtils.newFloatBuffer(arr.length);
        buff.put(arr);
        buff.flip();
        return buff;
    }

    @Override
    public void dispose() {
        Gdx.gl.glDeleteBuffer(VBO);
        Gdx.gl30.glDeleteVertexArrays(1, new int[VAO], 0);
        Gdx.gl.glDeleteTexture(cubeMap);
    }
}
