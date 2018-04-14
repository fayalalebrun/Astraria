package com.mygdx.game.simulation.renderer;

import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.graphics.Color;
import com.badlogic.gdx.utils.BufferUtils;
import com.badlogic.gdx.utils.Disposable;
import org.joml.Vector4f;

import java.nio.FloatBuffer;

import static com.badlogic.gdx.graphics.GL20.GL_ARRAY_BUFFER;

/**
 * Created by Fran on 4/13/2018.
 */
public class Line implements Disposable{
    private int VAO;
    private int VBO;

    private final FloatBuffer vertices;

    private int maxPoints;

    private final Vector4f color;

    private int drawUntil = 0;

    public Line(int maxPoints, Color color) {
        this.maxPoints = maxPoints;
        vertices = BufferUtils.newFloatBuffer(maxPoints*3);

        this.color = new Vector4f(color.r, color.g, color.b, color.a);

        setup();
    }

    private void setup(){
        int[] tempv = new int[1];
        Gdx.gl30.glGenVertexArrays(1, tempv, 0);
        VAO = tempv[0];


        Gdx.gl30.glBindVertexArray(VAO);

        VBO = Gdx.gl.glGenBuffer();
        Gdx.gl.glBindBuffer(GL_ARRAY_BUFFER, VBO);
        Gdx.gl.glBufferData(GL_ARRAY_BUFFER, maxPoints*3*4, vertices, Gdx.gl.GL_STREAM_DRAW);
        Gdx.gl.glEnableVertexAttribArray(0);
        Gdx.gl.glVertexAttribPointer(0, 3, Gdx.gl.GL_FLOAT, false, 0, 0);

        Gdx.gl.glDisableVertexAttribArray(0);

        Gdx.gl.glBindBuffer(GL_ARRAY_BUFFER, 0);
        Gdx.gl30.glBindVertexArray(0);
    }

    public void render(Shader shader){
        Gdx.gl.glLineWidth(1.5f);

        shader.use();
        shader.setVec4f("color", color);

        Gdx.gl30.glBindVertexArray(VAO);

        Gdx.gl.glEnableVertexAttribArray(0);


        Gdx.gl.glDrawArrays(Gdx.gl.GL_LINE_STRIP, 0, drawUntil);

        Gdx.gl.glDisableVertexAttribArray(0);

        Gdx.gl30.glBindVertexArray(0);
    }

    public void updateBufferData(float[] vertices, int numberOfPoints){
        this.vertices.rewind();
        this.vertices.put(vertices);
        this.vertices.flip();

        Gdx.gl.glBindBuffer(Gdx.gl.GL_ARRAY_BUFFER, VBO);
        Gdx.gl.glBufferData(Gdx.gl.GL_ARRAY_BUFFER, maxPoints*3*4, null, Gdx.gl.GL_STREAM_DRAW);
        Gdx.gl.glBufferSubData(Gdx.gl.GL_ARRAY_BUFFER,0,maxPoints*3*4,this.vertices);

        drawUntil = numberOfPoints;
    }

    @Override
    public void dispose() {
        Gdx.gl30.glDeleteVertexArrays(1,new int[VAO], 0);
        Gdx.gl.glDeleteBuffer(VBO);
    }
}
