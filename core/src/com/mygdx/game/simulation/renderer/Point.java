package com.mygdx.game.simulation.renderer;


import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.utils.BufferUtils;
import com.badlogic.gdx.utils.Disposable;
import org.joml.Vector3d;
import org.joml.Vector3f;
import org.joml.Vector4f;

import java.nio.FloatBuffer;
import java.nio.IntBuffer;

import static com.badlogic.gdx.graphics.GL20.GL_ARRAY_BUFFER;

public class Point implements Disposable{
    protected float[] vertices;

    private int VAO;
    private int EBO;

    private int verticeVBO;

    private final Vector3d position, temp;

    private final Vector3f rotation, temp2;

    private final Vector4f color;

    private final Transformation transformation;

    public Point(Vector4f color, Transformation transformation) {
        this.position = new Vector3d();
        rotation = new Vector3f();

        this.color = new Vector4f().set(color);

        temp = new Vector3d();
        temp2 = new Vector3f();

        this.transformation = transformation;

        setup();
    }

    private void setup(){
        vertices = new float[]{0,0,0};

        int[] tempv = new int[1];
        Gdx.gl30.glGenVertexArrays(1, tempv, 0);
        VAO = tempv[0];

        Gdx.gl30.glBindVertexArray(VAO);

        verticeVBO = Gdx.gl.glGenBuffer();
        Gdx.gl.glBindBuffer(GL_ARRAY_BUFFER, verticeVBO);
        Gdx.gl.glBufferData(GL_ARRAY_BUFFER, vertices.length*4, toFloatBuffer(vertices), Gdx.gl.GL_STATIC_DRAW);
        Gdx.gl.glEnableVertexAttribArray(0);
        Gdx.gl.glVertexAttribPointer(0, 3, Gdx.gl.GL_FLOAT, false, 0, 0);

        Gdx.gl.glDisableVertexAttribArray(0);
    }

    public void render(Shader shader, Camera cam){
        shader.use();
        shader.setMat4("modelView", transformation.getModelViewMatrix(transformation.getViewMatrix(cam),getPositionRelativeToCamera(cam),rotation, 1));
        shader.setVec4f("color", color);

        Gdx.gl30.glBindVertexArray(VAO);

        Gdx.gl.glEnableVertexAttribArray(0);

        Gdx.gl.glDrawArrays(Gdx.gl.GL_POINTS, 0, 1);

        Gdx.gl.glDisableVertexAttribArray(0);

        Gdx.gl30.glBindVertexArray(0);
    }

    private void setPosition(int x, int y, int z){
        this.position.set(x,y,z);
    }

    @Override
    public void dispose() {
        Gdx.gl.glDeleteBuffer(verticeVBO);
        Gdx.gl30.glDeleteVertexArrays(0,new int[]{VAO}, 0);
    }

    public Vector3f getPositionRelativeToCamera(Camera cam){
        return temp2.set(temp.set(position).sub(cam.getPosition()));
    }

    protected IntBuffer toIntBuffer(int[] arr){
        IntBuffer buff = BufferUtils.newIntBuffer(arr.length);
        buff.put(arr);
        buff.flip();
        return buff;
    }

    protected FloatBuffer toFloatBuffer(float[] arr){
        FloatBuffer buff = BufferUtils.newFloatBuffer(arr.length);
        buff.put(arr);
        buff.flip();
        return buff;
    }
}
