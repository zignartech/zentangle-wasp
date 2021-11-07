import { ComponentFixture, TestBed } from '@angular/core/testing';

import { IcentivizedAITrainingComponent } from './icentivized-aitraining.component';

describe('IcentivizedAITrainingComponent', () => {
  let component: IcentivizedAITrainingComponent;
  let fixture: ComponentFixture<IcentivizedAITrainingComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ IcentivizedAITrainingComponent ]
    })
    .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(IcentivizedAITrainingComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
