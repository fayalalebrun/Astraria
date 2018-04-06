package com.mygdx.game.simulation.renderer;

import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.utils.BufferUtils;
import com.badlogic.gdx.utils.Disposable;
import com.mygdx.game.simulation.Star;
import org.joml.Vector3d;
import org.joml.Vector3f;

import java.nio.FloatBuffer;
import java.nio.IntBuffer;
import java.util.ArrayList;
import java.util.List;

import static com.badlogic.gdx.graphics.GL20.GL_ARRAY_BUFFER;

public class LensGlow implements Disposable {
    private int textureID, spectrumTexID;

    private int VAO, EBO;

    private int[] indices;
    private float[] vertices;
    private float[] texCoords;

    private List<Integer> vboList;

    private Star star;

    private final Vector3d position, temp;

    private Transformation transformation;

    private final Vector3f temp2, rotation;


    public LensGlow(double x, double y, double z, int textureID, int spectrumTexID, Star star, Transformation transformation) {
        this.textureID = textureID;
        this.spectrumTexID = spectrumTexID;
        this.star = star;
        this.position = new Vector3d(x,y,z);
        this.transformation = transformation;

        this.temp = new Vector3d();
        this.temp2 = new Vector3f();
        this.rotation = new Vector3f();

        this.vboList = new ArrayList<Integer>();

        this.setup();
    }

    private void setup(){
        indices = new int[]{0,2,1,1,2,3};
        vertices = new float[]{
                -1, 1, 0,
                1,1,0,
                -1,-1,0,
                1,-1,0};
        texCoords = new float[]{
                0,1,
                1,1,
                0,0,
                1,0
        };

        int[] tempv = new int[1];
        Gdx.gl30.glGenVertexArrays(1, tempv, 0);
        VAO = tempv[0];


        Gdx.gl30.glBindVertexArray(VAO);

        int VBO = Gdx.gl.glGenBuffer();
        vboList.add(VBO);
        Gdx.gl.glBindBuffer(GL_ARRAY_BUFFER, VBO);
        Gdx.gl.glBufferData(GL_ARRAY_BUFFER, vertices.length*4, toFloatBuffer(vertices), Gdx.gl.GL_STATIC_DRAW);
        Gdx.gl.glEnableVertexAttribArray(0);
        Gdx.gl.glVertexAttribPointer(0, 3, Gdx.gl.GL_FLOAT, false, 0, 0);

        VBO = Gdx.gl.glGenBuffer();
        vboList.add(VBO);
        Gdx.gl.glBindBuffer(GL_ARRAY_BUFFER, VBO);
        Gdx.gl.glBufferData(GL_ARRAY_BUFFER, texCoords.length*4, toFloatBuffer(texCoords), Gdx.gl.GL_STATIC_DRAW);
        Gdx.gl.glEnableVertexAttribArray(1);
        Gdx.gl.glVertexAttribPointer(1, 2, Gdx.gl.GL_FLOAT, false, 0, 0);

        EBO = Gdx.gl.glGenBuffer();
        Gdx.gl.glBindBuffer(Gdx.gl.GL_ELEMENT_ARRAY_BUFFER, EBO);
        Gdx.gl.glBufferData(Gdx.gl.GL_ELEMENT_ARRAY_BUFFER, indices.length*4,toIntBuffer(indices), Gdx.gl.GL_STATIC_DRAW);

        Gdx.gl.glDisableVertexAttribArray(0);
        Gdx.gl.glDisableVertexAttribArray(1);
    }

    public void render(Shader shader, int screenWidth, int screenHeight, Camera cam){
        float size = (float)calculateGlowSize(star.getSize()*200,star.getTemperature(),getPositionRelativeToCamera(cam).length());

        shader.use();
        shader.setMat4("modelView", transformation.getModelViewMatrix(transformation.getViewMatrix(cam),getPositionRelativeToCamera(cam),rotation, 1));
        shader.setFloat("width", size);
        shader.setFloat("height", size * (screenWidth/screenHeight));
        shader.setFloat("screenWidth", screenWidth);
        shader.setFloat("screenHeight", screenHeight);
        shader.setVec3f("uPos", getPositionRelativeToCamera(cam));
        shader.setFloat("temperature", star.getTemperature());


        Gdx.gl30.glBindVertexArray(VAO);

        Gdx.gl.glEnableVertexAttribArray(0);
        Gdx.gl.glEnableVertexAttribArray(1);

        Gdx.gl.glActiveTexture(Gdx.gl.GL_TEXTURE0);
        shader.setInt("tex", 0);
        Gdx.gl.glBindTexture(Gdx.gl.GL_TEXTURE_2D, textureID);

        Gdx.gl.glActiveTexture(Gdx.gl.GL_TEXTURE1);
        shader.setInt("spectrumTex",1);
        Gdx.gl.glBindTexture(Gdx.gl.GL_TEXTURE_2D, spectrumTexID);

        Gdx.gl.glActiveTexture(Gdx.gl.GL_TEXTURE0);

        Gdx.gl.glBindBuffer(Gdx.gl.GL_ELEMENT_ARRAY_BUFFER, EBO);
        Gdx.gl.glDrawElements(Gdx.gl.GL_TRIANGLES, indices.length, Gdx.gl.GL_UNSIGNED_INT, 0);

        Gdx.gl.glDisableVertexAttribArray(0);
        Gdx.gl.glDisableVertexAttribArray(1);

        Gdx.gl30.glBindVertexArray(0);
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

    public Vector3f getPositionRelativeToCamera(Camera cam){
        return temp2.set(temp.set(position).sub(cam.getPosition()));
    }

    public double calculateGlowSize(double diameter, double temperature, double distance) {
        final double DSUN = 1392684.0;
        final double TSUN = 5778.0;

        double d = distance; // Distance
        double D = diameter * DSUN;
        double L = (D * D) * Math.pow(temperature / TSUN, 4.0); // Luminosity
        return 0.016 * Math.pow(L, 0.25) / Math.pow(d, 0.3); // Size
    }

    public void setPosition(double x, double y, double z){
        position.set(x,y,z);
    }

    @Override
    public void dispose() {
        for(int i : vboList){
            Gdx.gl.glDeleteBuffer(i);
        }
        Gdx.gl.glDeleteBuffer(EBO);
        Gdx.gl30.glDeleteVertexArrays(1, new int[VAO], 0);
    }
}
